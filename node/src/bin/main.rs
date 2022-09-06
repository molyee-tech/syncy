use syncy_node::*;
use tokio::{mpsc, io, task};
use libp2p::identity::Keypair;
use libp2p::{SwarmEvent, SwarmBuilder, Swarm};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let (tx, rx) = mpsc::unbounded_channel();
    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&KEYS)
        .expect("can create auth keys");

    let upversion = upgrade::Version::V1;
    let creds = NoiseConfig::xx(auth_keys).into_authenticated();
    let mplex = mplex::MplexConfig::new();

    // todo build transport from config
    let transport = TokioTcpConfig::new()
        .upgrade(upversion)
        .authenticate(creds)
        .multiplex(mplex)
        .boxed();
    let mut stdin = io::BufReader::new(tokio::io::stdin()).lines().fuse();

    let mut swarm = {
        // Create a Kademlia behaviour.
        let behavior = behavior::Builder::new(local_peer_id, local_key).build();
        Swarm::new(transport, behaviour, local_peer_id)
    };
    let addr = "/ip4/0.0.0.0/tcp/0".parse().expect("can get a local socket");
    Swarm::listen_on(&mut swarm, addr).expect("swarm can be started");

    loop {
        select! {
        line = stdin.select_next_some() => swarm.behaviour_mut().handle_input(&line.expect("Stdin not to close")),
        event = swarm.select_next_some() => match event {
            SwarmEvent::NewListenAddr { addr, .. } => println!("Listening in {:?}", addr),
            SwarmEvent::Behaviour(event) => swarm.behaviour_mut().handle_event(event),
            _ => {}
        }
    }
}
