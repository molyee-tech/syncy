use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(flatten)]
    pub root: RootOpts,
    #[clap(subcommand)]
    pub command: Cmd,
}

#[derive(Parser, Debug)]
pub struct RootOpts {
    //#[clap(flatten)]
    //pub target: ClusterTarget,
}

#[derive(Parser, Debug)]
pub enum Cmd {
    Client(ClientOpts),
    Hub(HubOpts),
    Storage(StorageOpts),
} 

#[derive(Parser, Debug)]
pub struct ClientOpts {

}

#[derive(Parser, Debug)]
pub struct HubOpts {

}

#[derive(Parser, Debug)]
pub struct StorageOpts {

}
