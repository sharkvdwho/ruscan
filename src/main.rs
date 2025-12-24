mod args;
mod network;
mod report;
mod service;

use args::*;
use clap::Parser;
use std::sync::{Arc, Mutex};
use std::thread;
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};
use std::str::FromStr;
use chrono::{Local, Datelike, Timelike};
use dns_lookup::lookup_host;
use report::{ReportGenerator, ScanResult};
use network::{parse_cidr, parse_ip_range, is_valid_ip};

const RED : &str = "\x1b[31m";
const GREEN : &str = "\x1b[32m";
const BLUE : &str = "\x1b[34m";
const RESET : &str = "\x1b[0m";
const STARING: &str = r"
 ____  _   _ ____   ____    _    _   _ 
|  _ \| | | / ___| / ___|  / \  | \ | |
| |_) | | | \___ \| |     / _ \ |  \| |
|  _ <| |_| |___) | |___ / ___ \| |\  |
|_| \_\\___/|____/ \____/_/   \_\_| \_|

                                      ";

fn init_port_scan(
    addrs: Vec<String>,
    ports: Vec<u16>,
    service_detection: bool,
    max_threads: usize,
    reporter: Arc<Mutex<ReportGenerator>>,
    only_open: bool,
) {
    let semaphore = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for addr in addrs {
        for port in &ports {
            // Rate control: wait if we have too many threads
            loop {
                let count = *semaphore.lock().unwrap();
                if count < max_threads {
                    *semaphore.lock().unwrap() = count + 1;
                    break;
                }
                thread::sleep(Duration::from_millis(10));
            }

            let addr_clone = addr.clone();
            let port_clone = *port;
            let reporter_clone = reporter.clone();
            let semaphore_clone = semaphore.clone();

            let handle = thread::spawn(move || {
                let host = format!("{}:{}", addr_clone, port_clone);
                let socket_addr = match SocketAddr::from_str(&host) {
                    Ok(addr) => addr,
                    Err(_) => {
                        *semaphore_clone.lock().unwrap() -= 1;
                        return;
                    }
                };

                let (status, service, version) = match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3)) {
                    Ok(_) => {
                        if service_detection {
                            let (svc, ver) = service::detect_service(&socket_addr);
                            (String::from("open"), svc, ver)
                        } else {
                            (String::from("open"), None, None)
                        }
                    }
                    Err(_) => (String::from("closed"), None, None),
                };

                // Print result (only open ports if scanning multiple IPs)
                if status == "open" {
                    let service_info = if let Some(ref svc) = service {
                        if let Some(ref ver) = version {
                            format!(" ({}/{})", svc, ver)
                        } else {
                            format!(" ({})", svc)
                        }
                    } else {
                        String::new()
                    };
                    println!("{}[+] {}:{} is open{}{}", GREEN, addr_clone, port_clone, service_info, RESET);
                } else if !only_open {
                    // Only print closed ports if not scanning multiple IPs
                    // println!("{}[-] {}:{} is closed{}", RED, addr_clone, port_clone, RESET);
                }

                // Add to report
                let result = ScanResult {
                    host: addr_clone,
                    port: port_clone,
                    status: status.clone(),
                    service: service.clone(),
                    version: version.clone(),
                };
                reporter_clone.lock().unwrap().add_result(result);

                *semaphore_clone.lock().unwrap() -= 1;
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main(){
    println!("\t{}{}{}",BLUE, STARING, RESET);
    let time = Local::now();
    let (hour, minute, second) = (time.hour(), time.minute(), time.second());
    let (year, month, day) = (time.year(), time.month(), time.day());
    println!("{}Starting Ruscan ( https://github.com/sharkvdwho/ruscan ) at {}-{:02}-{:02} {:02}:{:02}:{:02}{} \n",GREEN, year, month, day, hour, minute, second, RESET);
    let args = RuscanArgs::parse();

    match args.entity_type {
        EntityType::Ps(port_scan) => {
            let mut addrs: Vec<String> = Vec::new();

            // Handle single IP
            if let Some(ip) = port_scan.ip {
                if is_valid_ip(&ip) {
                    addrs.push(ip);
                } else {
                    println!("{}[-] Invalid IP address: {}{}", RED, ip, RESET);
                    return;
                }
            }

            // Handle domain
            if let Some(domain) = port_scan.domain {
                let resolved = lookup_host(&domain);
                match resolved {
                    Ok(res) => {
                        for ip in res {
                            addrs.push(ip.to_string());
                        }
                    },
                    Err(_) => {
                        println!("{}[-] Could not resolve the domain: {}{}", RED, domain, RESET);
                        return;
                    }
                }
            }

            // Handle CIDR notation
            if let Some(cidr) = port_scan.cidr {
                match parse_cidr(&cidr) {
                    Ok(ips) => {
                        println!("{}[+] Scanning {} IPs from CIDR: {}{}", GREEN, ips.len(), cidr, RESET);
                        addrs.extend(ips);
                    },
                    Err(e) => {
                        println!("{}[-] Error parsing CIDR: {}{}", RED, e, RESET);
                        return;
                    }
                }
            }

            // Handle IP range
            if let Some(ip_range) = port_scan.ip_range {
                match parse_ip_range(&ip_range) {
                    Ok(ips) => {
                        println!("{}[+] Scanning {} IPs from range: {}{}", GREEN, ips.len(), ip_range, RESET);
                        addrs.extend(ips);
                    },
                    Err(e) => {
                        println!("{}[-] Error parsing IP range: {}{}", RED, e, RESET);
                        return;
                    }
                }
            }

            if addrs.is_empty() {
                println!("{}[-] No target address specified. Use -i, -d, -c, or -R option.{}", RED, RESET);
                return;
            }

            // Determine ports to scan
            let mut ports: Vec<u16> = Vec::new();

            if let Some(port) = port_scan.port {
                ports.push(port);
            }

            if let Some(range) = port_scan.range {
                let mut split = range.split("-");
                let start: u16 = split.next()
                    .expect(&format!("{}[-] Invalid range value, see ruscan ps --help for more information{}", RED, RESET))
                    .parse()
                    .unwrap();
                let end: u16 = split.next()
                    .expect(&format!("{}[-] Invalid range value, see ruscan ps --help for more information{}", RED, RESET))
                    .parse()
                    .unwrap();
                ports.extend(start..=end);
            }

            if let Some(list) = port_scan.list {
                let mut split = list.split(",");
                while let Some(v) = split.next() {
                    if let Ok(port) = v.trim().parse::<u16>() {
                        ports.push(port);
                    }
                }
            }

            // Default to common ports if none specified
            if ports.is_empty() {
                ports = vec![21, 22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 3306, 3389, 5432, 8080];
                println!("{}[+] No ports specified, scanning common ports{}", GREEN, RESET);
            }

            // Initialize reporter
            let reporter = Arc::new(Mutex::new(ReportGenerator::new()));

            // Determine if we should only show open ports
            // Show only open ports if: scanning multiple IPs OR scanning a large port range (>100 ports)
            let total_scans = addrs.len() * ports.len();
            let only_open = addrs.len() > 1 || ports.len() > 100;

            // Perform scan
            println!("{}[+] Starting scan of {} host(s) on {} port(s) ({} total connections){}", 
                GREEN, addrs.len(), ports.len(), total_scans, RESET);
            if only_open {
                if addrs.len() > 1 {
                    println!("{}[+] Showing only open ports (scanning multiple IPs){}", BLUE, RESET);
                } else {
                    println!("{}[+] Showing only open ports (scanning large port range){}", BLUE, RESET);
                }
            }
            println!("{}[+] Scanning...{}", BLUE, RESET);
            
            // Start timing the scan
            let start_time = Instant::now();
            
            init_port_scan(
                addrs.clone(),
                ports,
                port_scan.service_detection,
                port_scan.threads,
                reporter.clone(),
                only_open,
            );

            // Calculate elapsed time in seconds with millisecond precision (3 decimal places)
            let elapsed = start_time.elapsed().as_secs_f64();
            println!("{}[+] Scan completed in {:.3} seconds{}", GREEN, elapsed, RESET);

            // Generate report (only if saving to file or using non-text format)
            let reporter_guard = reporter.lock().unwrap();
            if port_scan.file.is_some() || port_scan.output.to_lowercase() != "text" {
                if let Err(e) = reporter_guard.generate(&port_scan.output, port_scan.file.as_deref()) {
                    println!("{}[-] Error generating report: {}{}", RED, e, RESET);
                }
            }
        },
    }
}
