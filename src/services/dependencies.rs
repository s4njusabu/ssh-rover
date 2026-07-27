use std::{fs, process::Command};

// Get OS name
// arch for archlinux
// ubuntu for ubuntu
pub fn get_os_id() -> std::io::Result<String> {
    let output = fs::read_to_string("/etc/os-release")?;

    for line in output.lines() {
        if let Some(id) = line.strip_prefix("ID=") {
            return Ok(id.trim_matches('"').to_string());
        }
    }

    Ok(String::from("NOT FOUND"))
}

// Installed or not
pub fn nmap_installed() -> bool {
    Command::new("nmap")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

pub fn openssh_installed() -> bool {
    Command::new("ssh")
        .arg("-V")
        .output()
        .is_ok_and(|output| output.status.success())
}
