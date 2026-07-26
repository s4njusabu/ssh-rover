use crate::services::discovery::network;

pub fn scan_current_network() -> Vec<String> {
    if let Some(interface) = super::network::get_interface()
        && let Some(cidr) = super::network::get_interface_cidr(&interface)
    {
        return network::scan_cidr_range(&cidr);
    }

    Vec::new()
}
