use std::{fs, process::Command};

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

// setup the systemd if not setup
