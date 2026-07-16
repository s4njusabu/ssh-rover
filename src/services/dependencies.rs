#![allow(dead_code)]

use std::fs;

pub const FAMILIES: [(&str, &str, &str, &str); 3] = [
    ("arch", "pacman", "nmap", "openssh"),
    ("debian", "apt", "nmap", "openssh-server"),
    ("rhel", "dnf", "nmap", "openssh"),
];

// Get OS name
// arch for archlinux
// ubuntu for ubuntu
fn get_os_id() -> std::io::Result<String> {
    let output = fs::read_to_string("/etc/os-release")?;

    for line in output.lines() {
        if let Some(id) = line.strip_prefix("ID=") {
            return Ok(id.trim_matches('"').to_string());
        }
    }

    Ok(String::from("ID not found"))
}

// setup the systemd if not setup
