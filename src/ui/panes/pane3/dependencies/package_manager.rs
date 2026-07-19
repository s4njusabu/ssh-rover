pub fn package_manager(name: &str) -> &'static str {
    match name {
        "arch" | "endeavouros" | "manjaro" | "garuda" | "artix" | "cachyos" | "arcolinux" => {
            "pacman"
        }
        "ubuntu" | "debian" | "linuxmint" | "pop" | "kali" | "raspbian" | "zorin"
        | "elementary" => "apt",
        "fedora" | "rhel" | "rocky" | "almalinux" | "centos" | "amzn" => "dnf",
        "opensuse-leap" | "opensuse-tumbleweed" | "sles" => "zypper",
        "alpine" => "apk",
        _ => "UNKNOWN",
    }
}

pub fn nmap_package_name(package_manager: &str) -> &'static str {
    match package_manager {
        "pacman" | "apt" | "dnf" | "zypper" | "apk" => "nmap",
        _ => "UNKNOWN",
    }
}

pub fn nmap_package_install(package_manager: &str) -> &'static str {
    match package_manager {
        "pacman" => "sudo pacman -S --needed nmap",
        "apt" => "sudo apt install nmap",
        "dnf" => "sudo dnf install nmap",
        "zypper" => "sudo zypper install nmap",
        "apk" => "sudo apk add nmap",
        _ => "UNKNOWN",
    }
}

pub fn openssh_package_name(name: &str) -> &'static str {
    match name {
        "pacman" | "apk" => "openssh",
        "apt" => "openssh-server",
        "dnf" => "openssh-clients openssh-server",
        "zypper" => "openssh openssh-server",
        _ => "UNKNOWN",
    }
}

pub fn openssh_package_install(name: &str) -> &'static str {
    match name {
        "pacman" => "sudo pacman -S --needed openssh",
        "apt" => "sudo apt install openssh-server",
        "dnf" => "sudo dnf install openssh-clients openssh-server",
        "zypper" => "sudo zypper install openssh openssh-server",
        "apk" => "sudo apk add openssh",
        _ => "UNKNOWN",
    }
}
