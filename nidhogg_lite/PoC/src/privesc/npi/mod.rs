use std::process::Command;

pub fn npi_check() {
    let cmd: &str = npi_xp_check;
    let mut command: String = cmd.to_owned();
    let cmd_addons: &str = "
    Get-RegistryAlwaysInstallElevated
    ";
    command.push_str(cmd_addons);
    let out = Command::new(command).output().expect("failed to execute process");
    println!("Vulnerable: {}", String::from_utf8_lossy(&out.stdout));
    
    if String::from_utf8_lossy(&out.stdout).contains("True"){
        npi_xp()
    }
}

fn npi_xp(){
    let cmd: &str = npi_xp_run;
    let mut command: String = cmd.to_owned();
    let cmd_addons: &str = "
    Some-PS-Function
    ";
    command.push_str(cmd_addons);
    let out = Command::new(command).output().expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&out.stdout));
}

static npi_xp_check: &'static str = "";

static npi_xp_run: &'static str = "";