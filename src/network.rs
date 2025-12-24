use ipnet::IpNet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Parse CIDR notation and return a list of IP addresses
pub fn parse_cidr(cidr_str: &str) -> Result<Vec<String>, String> {
    let network: IpNet = cidr_str.parse()
        .map_err(|_| format!("Invalid CIDR notation: {}", cidr_str))?;
    
    let mut ips = Vec::new();
    match network {
        IpNet::V4(net) => {
            for ip in net.hosts() {
                ips.push(ip.to_string());
            }
        }
        IpNet::V6(net) => {
            for ip in net.hosts() {
                ips.push(ip.to_string());
            }
        }
    }
    Ok(ips)
}

/// Parse IP range (e.g., "192.168.1.1-192.168.1.254")
pub fn parse_ip_range(range_str: &str) -> Result<Vec<String>, String> {
    let parts: Vec<&str> = range_str.split('-').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid IP range format: {}", range_str));
    }
    
    let start_ip: IpAddr = parts[0].trim().parse()
        .map_err(|_| format!("Invalid start IP: {}", parts[0]))?;
    let end_ip: IpAddr = parts[1].trim().parse()
        .map_err(|_| format!("Invalid end IP: {}", parts[1]))?;
    
    let mut ips = Vec::new();
    
    match (start_ip, end_ip) {
        (IpAddr::V4(start), IpAddr::V4(end)) => {
            let start_u32 = u32::from(start);
            let end_u32 = u32::from(end);
            
            if start_u32 > end_u32 {
                return Err("Start IP must be less than or equal to end IP".to_string());
            }
            
            for ip_u32 in start_u32..=end_u32 {
                ips.push(Ipv4Addr::from(ip_u32).to_string());
            }
        }
        (IpAddr::V6(start), IpAddr::V6(end)) => {
            let start_u128 = u128::from(start);
            let end_u128 = u128::from(end);
            
            if start_u128 > end_u128 {
                return Err("Start IP must be less than or equal to end IP".to_string());
            }
            
            // For IPv6, we limit the range to prevent excessive memory usage
            let diff = end_u128.saturating_sub(start_u128);
            if diff > 65536 {
                return Err("IPv6 range too large (max 65536 addresses)".to_string());
            }
            
            for ip_u128 in start_u128..=end_u128 {
                ips.push(Ipv6Addr::from(ip_u128).to_string());
            }
        }
        _ => return Err("Start and end IPs must be of the same version (IPv4 or IPv6)".to_string()),
    }
    
    Ok(ips)
}

/// Check if an IP address is valid (supports both IPv4 and IPv6)
pub fn is_valid_ip(ip_str: &str) -> bool {
    ip_str.parse::<IpAddr>().is_ok()
}

