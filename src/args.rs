use clap::{
    Args,
    Parser,
    Subcommand
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct RuscanArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType, 
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Perform network mapping of systems exposed to the Internet
    Ps(PortScan),
//     /// this going to contain other programs
//     UnderDevelopment(Enumuration)
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)] 
pub struct PortScan {
    /// IP address of the target network (IPv4 or IPv6). Example: ruscan ps -i 127.0.0.1
    #[arg(short, long)]
    pub ip: Option<String>,
    /// Domain name of the target network. Example: ruscan ps -d example.com
    #[arg(short, long)]
    pub domain: Option<String>,
    /// CIDR notation for network scanning (supports IPv4 and IPv6). Example: ruscan ps -c 192.168.1.0/24
    #[arg(short = 'c', long)]
    pub cidr: Option<String>,
    /// IP range for scanning (start-end format). Example: ruscan ps -R 192.168.1.1-192.168.1.254
    #[arg(short = 'R', long)]
    pub ip_range: Option<String>,
    /// Single port to be scanned on the target network. Example: ruscan ps -i 127.0.0.1 -p 443
    #[arg(short, long)]
    pub port: Option<u16>,
    /// Range of ports to be scanned on the target network. Example: ruscan ps -i 127.0.0.1 -r 1-65535
    #[arg(short, long)]
    pub range: Option<String>,
    /// List of ports to be scanned on the target network (comma-separated). Example: ruscan ps -i 127.0.0.1 -l 21,80,443
    #[arg(short, long)]
    pub list: Option<String>,
    /// Output format: json, csv, html, or text (default). Example: ruscan ps -i 127.0.0.1 -o json
    #[arg(short, long, default_value = "text")]
    pub output: String,
    /// Save results to a file. Example: ruscan ps -i 127.0.0.1 -f results.json
    #[arg(short, long)]
    pub file: Option<String>,
    /// Enable service and version detection (banner grabbing). Example: ruscan ps -i 127.0.0.1 -s
    #[arg(short = 's', long)]
    pub service_detection: bool,
    /// Maximum number of concurrent connections for rate control (default: 1000). Example: ruscan ps -i 127.0.0.1 -t 100
    #[arg(short = 't', long, default_value = "1000")]
    pub threads: usize,
}

#[derive(Debug, Args)]
pub struct Enumuration{

}