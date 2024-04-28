mod args;

use args::*;
use clap::Parser;
use std::thread::{JoinHandle, spawn};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::str::FromStr;
use chrono::{Local, Datelike, Timelike};
use dns_lookup::lookup_host; //lookup_addr for dns lookup from 

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

                          by sharkvdwho
                                      
                                      ";

fn init_port_scan(addr: &String, list: Vec<u16>){
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for port in list {
        let host: String = format!("{}:{}", addr, port);
        let handle: JoinHandle<()> = spawn(move || {
            let socker_addr: SocketAddr = SocketAddr::from_str(host.as_str()).unwrap();
            match TcpStream::connect_timeout(&socker_addr, Duration::from_secs(3)) {
                Ok(_) => println!("{}Port {} is open{}", GREEN, port, RESET),
                Err(_) => println!("{}Port {} is closed{}", RED, port, RESET),
            }
        });
        handles.push(handle);
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

            let mut addr: String = String::new(); 

            if let Some(ip) = port_scan.ip {
                addr = ip.to_string();
            }

            if let Some(domain) = port_scan.domain {
                addr = domain.to_string();
                let resolved = lookup_host(&addr);
                match resolved {
                    Ok(res) => {
                        addr = res[0].to_string();                     
                    },
                    Err(_) => {
                        println!("{}[-] Could not resolve the domain: {}{}",RED , &addr, RESET);
                        return;
                    }
                }
            }

            if let Some(port) = port_scan.port {
                init_port_scan(&addr, vec![port]);
            }

            if let Some(range) = port_scan.range {

                let mut split = range.split("-");

                let start: u16 = match split.next() {
                    Some(v) => v.parse().unwrap(),
                    None => panic!("{}[-] Invalid range value, see ruscan ps --help for more information{}", RED, RESET),
                };

                let end: u16 = match split.next() {
                    Some(v) => v.parse().unwrap(),
                    None => panic!("{}[-] Invalid range value, see ruscan ps --help for more information{}", RED, RESET),
                };

                let mut list: Vec<u16> = Vec::new();
                (start..=end).into_iter().for_each(|x| list.push(x));
                init_port_scan(&addr, list);
            }

            if let Some(list) = port_scan.list {

                let mut ports: Vec<u16> = Vec::new();
                let mut split = list.split(",");

                loop {
                    match split.next() {
                        Some(v) => ports.push(v.parse().unwrap()),
                        None => break,
                    }
                }
                init_port_scan(&addr, ports);
            }

        },
        EntityType::UnderDevelopment(_) => {
            // TODO
            println!("{}This program currently under development{}", RED, RESET);
        },
    }
}