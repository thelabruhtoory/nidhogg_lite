# Overview

Nidhogg_Lite is a PoC for malware.
## Functionality

- [x] Remote Access

Data Exfiltration:

- [x] Keylogging
- [x] Google Chrome Passwords 
- [x] Firefox Passwords

Privilege Escalation:

- [ ] Always Install Elevated
- [ ] Weak Service Paths
- [ ] Named Pipe Impersonation

Persistence:

- [ ] Registry Persistence
- [ ] Reflective Dll Injection

AV Evasion:

- [ ] VirusTotal
- [ ] Cuckoo Sandbox

## Build Notes

When recieving the following error:

```txt
3 | #![feature(proc_macro_span)]
  |            ^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0554`.
```

Run the following:

```bash
rustup install nightly
rustup override set nightly
```

### References

I definietly could not have built this project in the time frame that I did without these projects:

- [Rust Reverse TCP Shell](https://github.com/CypElf/Reverse-shell)
- [Firefox Passwords Python Script](https://github.com/Unode/firefox_decrypt) 
- [Chrome Passwords Python Script](https://github.com/ohyicong/decrypt-chrome-passwords)
- [Windows Registry Interaction in Rust](https://docs.rs/winreg/latest/winreg/)
- [PowerUp Powershell Script](https://raw.githubusercontent.com/PowerShellMafia/PowerSploit/master/Privesc/PowerUp.ps1)
- [Basic Rust Keylogger](https://github.com/thomaslienbacher/win-keylogger-rs)