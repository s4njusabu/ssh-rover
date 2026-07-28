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

pub fn scan_cidr_range(cidr: &str) -> Vec<String> {
    let net: Ipv4Net = match cidr.trim().parse() {
        Ok(net) => net,
        Err(_) => return Vec::new(),
    };

    let network = net.network().to_string();
    let prefix_len = net.prefix_len().to_string();

    let argument_for_nmap = format!("{network}/{prefix_len}");

    match Command::new("nmap")
        .args(["-sn", argument_for_nmap.as_str()])
        .output()
    {
        Ok(content) => String::from_utf8_lossy(&content.stdout)
            .lines()
            .filter_map(|line| line.strip_prefix("Nmap scan report for "))
            .map(|val| val.to_string())
            .collect(),
        Err(_) => Vec::new(),
    }
}
