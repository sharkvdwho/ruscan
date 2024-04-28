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
    /// this going to contain other programs
    Todo(Enumuration)
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)] 
pub struct PortScan {
    /// -i --ip <ip_address> | ip addres of the target network. Example: ruscan ps <127.0.0.1>
    #[arg(short, long)]
    pub ip: Option<String>,
    /// -d --domain <domain_name> | domain name of the target network. Example: ruscan ps <localhost>
    #[arg(short, long)]
    pub domain: Option<String>,
    /// -p --port <single_port> | single port to be scaned on the target network. Example: ruscan ps target_address -p <443>
    #[arg(short, long)]
    pub port: Option<u16>,
    /// -r --range <range_of_ports> | range of ports to be scaned on the target network. Example: ruscan ps target_address -r <1-65535>
    #[arg(short, long)]
    pub range: Option<String>,
    /// -l --list <list_of_prots> | list of ports to be scaned on the target network. Example: ruscan ps target_address -l <21,80,443>
    #[arg(short, long)]
    pub list: Option<String>,
}

#[derive(Debug, Args)]
pub struct Enumuration{

}