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
    UnderDevelopment(Enumuration)
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)] 
pub struct PortScan {
    /// ip addres of the target network. Example: ruscan ps <127.0.0.1>
    #[arg(short, long)]
    pub ip: Option<String>,
    /// domain name of the target network. Example: ruscan ps <localhost>
    #[arg(short, long)]
    pub domain: Option<String>,
    /// single port to be scaned on the target network. Example: ruscan ps target_address -p <443>
    #[arg(short, long)]
    pub port: Option<u16>,
    /// range of ports to be scaned on the target network. Example: ruscan ps target_address -r <1-65535>
    #[arg(short, long)]
    pub range: Option<String>,
    /// list of ports to be scaned on the target network. Example: ruscan ps target_address -l <21,80,443>
    #[arg(short, long)]
    pub list: Option<String>,
}

#[derive(Debug, Args)]
pub struct Enumuration{

}