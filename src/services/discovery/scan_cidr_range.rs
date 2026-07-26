use std::process::Command;

use ipnet::Ipv4Net;

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
