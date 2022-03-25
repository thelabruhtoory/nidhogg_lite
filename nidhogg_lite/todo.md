## Modular Functionality

- [ ] Remote Access
    - [x] standalone PoC
    - [x] basic reverse shell [CrypElf/Reverse-shell](https://github.com/CypElf/Reverse-shell)
    - [ ] handle custom commands
    - [ ] background shell when clicking .exe
    - [ ] polished framework integration

Data Exfiltration:

- [ ] Keylogging
    - [x] standalone PoC [thomaslienbacher/win-keylogger-rs](https://github.com/thomaslienbacher/win-keylogger-rs)
    - [x] output to file
    - [x] background process
    - [ ] polished framework integration

- [ ] Google Chrome Passwords [ohyicong/decrypt-chrome-passwords](https://github.com/ohyicong/decrypt-chrome-passwords)
    - [x] interpret python code 
    - [x] output to file
    - [ ] polished framework integration

- [ ] Firefox Passwords
    - [x] interpret python code [uncode/firefox_decrypt](https://github.com/Unode/firefox_decrypt) 
    - [x] output to file
    - [ ] polished framework integration

Privilege Escalation:

- [ ] Always Install Elevated
    - [x] xp checker
    - [x] xp run
    - [ ] polished framework integration

- [ ] Weak Service Paths
    - [x] xp checker
    - [x] xp run
    - [ ] polished framework integration

- [ ] Named Pipe Impersonation
    - [ ] xp checker
    - [ ] xp run
    - [ ] polished framework integration

Persistence:

- [ ] Registry Persistence
    - [x] Standalone PoC with powershell
    - [ ] Standalone PoC in Rust 
    - [ ] polished framework integration

- [ ] Reflective Dll Injection
    - [ ] Standalone PoC
    - [ ] polished framework integration

AV Evasion Checks:

- [ ] VirusTotal
- [ ] Cuckoo Sandbox
- [ ] Windows Defender