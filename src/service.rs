use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::time::Duration;

/// Attempt to detect service and version by banner grabbing
pub fn detect_service(addr: &SocketAddr) -> (Option<String>, Option<String>) {
    let mut stream = match TcpStream::connect_timeout(addr, Duration::from_secs(2)) {
        Ok(s) => s,
        Err(_) => return (None, None),
    };

    stream.set_read_timeout(Some(Duration::from_secs(2))).ok();
    
    // Try to read banner
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap_or(0);
    
    if bytes_read > 0 {
        let banner = String::from_utf8_lossy(&buffer[..bytes_read]);
        let (service, version) = parse_banner(&banner);
        return (Some(service), version);
    }

    // Try sending common probes for different services (need new connection for each)
    let probes: Vec<(&str, Vec<u8>)> = vec![
        ("HTTP", b"GET / HTTP/1.0\r\n\r\n".to_vec()),
        ("FTP", b"USER anonymous\r\n".to_vec()),
        ("SSH", b"SSH-2.0-Ruscan\r\n".to_vec()),
    ];

    for (service_name, probe) in probes {
        // Create a new connection for each probe
        if let Ok(mut probe_stream) = TcpStream::connect_timeout(addr, Duration::from_secs(2)) {
            probe_stream.set_read_timeout(Some(Duration::from_secs(2))).ok();
            if let Ok(_) = probe_stream.write_all(&probe) {
                if let Ok(_) = probe_stream.flush() {
                    let mut response = [0u8; 512];
                    if let Ok(n) = probe_stream.read(&mut response) {
                        if n > 0 {
                            let response_str = String::from_utf8_lossy(&response[..n]);
                            let (service, version) = parse_banner(&response_str);
                            if service != "Unknown" {
                                return (Some(service), version);
                            }
                            return (Some(service_name.to_string()), None);
                        }
                    }
                }
            }
        }
    }

    (None, None)
}

fn parse_banner(banner: &str) -> (String, Option<String>) {
    let banner_lower = banner.to_lowercase();
    
    // HTTP detection
    if banner.contains("HTTP/") {
        if let Some(version) = extract_http_version(banner) {
            return ("HTTP".to_string(), Some(version));
        }
        return ("HTTP".to_string(), None);
    }
    
    // SSH detection
    if banner.contains("SSH-") {
        if let Some(version) = extract_ssh_version(banner) {
            return ("SSH".to_string(), Some(version));
        }
        return ("SSH".to_string(), None);
    }
    
    // FTP detection
    if banner.contains("220") && banner_lower.contains("ftp") {
        return ("FTP".to_string(), None);
    }
    
    // SMTP detection
    if banner.contains("220") && banner_lower.contains("smtp") {
        return ("SMTP".to_string(), None);
    }
    
    // Telnet detection
    if banner_lower.contains("telnet") {
        return ("Telnet".to_string(), None);
    }
    
    ("Unknown".to_string(), None)
}

fn extract_http_version(banner: &str) -> Option<String> {
    if let Some(line) = banner.lines().next() {
        if let Some(version_start) = line.find("HTTP/") {
            let version_part = &line[version_start + 5..];
            if let Some(space_pos) = version_part.find(' ') {
                return Some(version_part[..space_pos].trim().to_string());
            }
            return Some(version_part.trim().to_string());
        }
    }
    None
}

fn extract_ssh_version(banner: &str) -> Option<String> {
    if let Some(line) = banner.lines().next() {
        if let Some(ssh_start) = line.find("SSH-") {
            let version_part = &line[ssh_start + 4..];
            if let Some(dash_pos) = version_part.find('-') {
                return Some(version_part[..dash_pos].trim().to_string());
            }
            if let Some(space_pos) = version_part.find(' ') {
                return Some(version_part[..space_pos].trim().to_string());
            }
            return Some(version_part.trim().to_string());
        }
    }
    None
}

