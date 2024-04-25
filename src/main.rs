
mod args; 

use args::*;
use clap::Parser;
use std::thread::{JoinHandle, spawn};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::str::FromStr;

fn init_port_scan(addr: &String, list: Vec<u16>){
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for port in list {
        let host: String = format!("{}:{}", addr, port);
        let handle: JoinHandle<()> = spawn(move || {
            let socker_addr: SocketAddr = SocketAddr::from_str(host.as_str()).unwrap();
            match TcpStream::connect_timeout(&socker_addr, Duration::from_secs(3)) {
                Ok(_) => println!("Port {} is open", port),
                Err(_) => println!("Port {} is closed", port),
            }
        });
        handles.push(handle);
    }  

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main(){
    let args = RuscanArgs::parse();

    match args.entity_type {
        EntityType::Ps(port_scan) => {

            let addr = port_scan.addr; 

            if let Some(port) = port_scan.port {
                init_port_scan(&addr, vec![port]);
            }

            if let Some(range) = port_scan.range {

                let mut split = range.split("-");

                let start: u16 = match split.next() {
                    Some(v) => v.parse().unwrap(),
                    _ => panic!("[-] Invalid range value, see ruscan ps --help for more information"),
                };

                let end: u16 = match split.next() {
                    Some(v) => v.parse().unwrap(),
                    _ => panic!("[-] Invalid range value, see ruscan ps --help for more information"),
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
        EntityType::Todo(_) => {
          // Handle this case (if needed)
        },
    }
}