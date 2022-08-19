use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(flatten)]
    pub root: RootOpts,
    #[clap(subcommand)]
    pub command: Cmd,
}

#[derive(Parser, Debug)]
pub struct RootOpts {
    #[clap(help = "Preconfigured network profile name")]
    pub profile: Option<String>,
    #[clap(help = "Path to custom configuration file (overlaps 'profile' option)")]
    pub config: Option<PathBuf>
}

#[derive(Parser, Debug)]
pub enum Cmd {
    #[clap(help = "Commands relative to local client daemon")]
    Client(ClientOpts),
    #[clap(help = "Commands relative to network hub (from profile settings)")]
    Hub(HubOpts),
    #[clap(alias = "network", help = "Commands relative to network cluster (information from hubs)")]
    Network(NetworkOpts),
    #[clap(help = "Commands relative to network storage nodes")]
    Storage(StorageOpts),
} 

#[derive(Parser, Debug)]
pub struct ClientOpts {
    #[clap(subcommand)]
    pub command: ClientCmd,
}

#[derive(Parser, Debug)]
pub enum ClientCmd {
    #[clap(help = "Get client daemon status")]
    Status(ClientStatusOpts),
    #[clap(help = "Connect to network hub with selected profile or config file")]
    Connect(ClientConnectOpts),
    #[clap(help = "Display detailed information on the client")]
    Inspect(ClientInspectOpts),
} 

#[derive(Parser, Debug)]
pub struct ClientStatusOpts {
}

#[derive(Parser, Debug)]
pub struct ClientConnectOpts {
}

#[derive(Parser, Debug)]
pub struct ClientInspectOpts {
}

#[derive(Parser, Debug)]
pub struct HubOpts {
    #[clap(subcommand)]
    pub command: HubCmd,
}

#[derive(Parser, Debug)]
pub enum HubCmd {
    #[clap(help = "Display default or specific network hub status")]
    Status(HubStatusOpts),
    #[clap(alias = "ls", help = "Display network hub list (based on profile or config)")]
    List(HubListOpts),
    #[clap(help = "Display detailed information on the specific network hub")]
    Inspect(HubInspectOpts),
} 

#[derive(Parser, Debug)]
pub struct HubStatusOpts {
}

#[derive(Parser, Debug)]
pub struct HubListOpts {
}

#[derive(Parser, Debug)]
pub struct HubInspectOpts {
}

#[derive(Parser, Debug)]
pub struct NetworkOpts {
    #[clap(subcommand)]
    pub command: NetCmd,
}

#[derive(Parser, Debug)]
pub enum NetCmd {
    Lookup(LookupOpts)
}

#[derive(Parser, Debug)]
pub struct LookupOpts {
    #[clap(help = "Filter searching elements")]
    pub filter: Vec<String>, // TODO
    #[clap(parse(from_str), help = "Name or link to a specific service or node")]
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct StorageOpts {

}
