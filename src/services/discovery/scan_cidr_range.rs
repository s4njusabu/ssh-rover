use ipnet::Ipv4Net;
use std::process::Command;

pub fn scan_cidr_range(cidr: &str) -> Vec<String> {
    let net: Ipv4Net = cidr.parse().unwrap();

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

fn auto() -> Vec<String> {
    if let Some(interface) = super::network::get_interface()
        && let Some(cidr) = super::network::get_interface_cidr(&interface)
    {
        return scan_cidr_range(&cidr);
    }

    Vec::new()
}
