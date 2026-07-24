use std::process::Command;

use ipnet::Ipv4Net;

pub fn get_interface() -> Option<String> {
    let output: Option<String> = match Command::new("ip")
        .args(["route", "get", "8.8.8.8"])
        .output()
    {
        Ok(content) => String::from_utf8(content.stdout).ok(),
        Err(_) => None,
    };

    if let Some(c1) = output {
        let mut words = c1.split_whitespace();

        while let Some(c2) = words.next() {
            if c2 == "dev" {
                return words.next().map(|c3| c3.to_string());
            }
        }
    }

    None
}

pub fn get_interface_cidr(interface: &str) -> Option<String> {
    let output = match Command::new("ip")
        .args(["addr", "show", interface])
        .output()
    {
        Ok(content) => String::from_utf8(content.stdout).ok(),
        Err(_) => None,
    };

    if let Some(c1) = output {
        let mut words = c1.split_whitespace();

        while let Some(c2) = words.next() {
            if c2 == "inet" {
                let network_addr = words.next()?;

                let net: Ipv4Net = network_addr.parse().ok()?;

                return Some(net.trunc().to_string());
            }
        }
    }

    None
}
