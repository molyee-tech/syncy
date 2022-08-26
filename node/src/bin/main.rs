use syncy_node::*;
use tokio::mpsc;
use libp2p::identity::Keypair;

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

    let transport = TokioTcpConfig::new()
        .upgrade(upversion)
        .authenticate(creds)
        .multiplex(mplex)
        .boxed();
    let mdns = TokioMdns::new().expect("can create mdns");
    let sub = Floodsub::new(PEER_ID.clone());
    let mut handler = NetHandler { sub, mdns, tx };
    handler.sub.subscribe(TOPIC.clone());

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    let mut swarm = SwarmBuilder::new(transport, handler, PEER_ID.clone())
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();
    
    let addr = "/ip4/0.0.0.0/tcp/3000".parse().expect("can get a local socket");
    Swarm::listen_on(&mut swarm, addr).expect("swarm can be started");

    loop {
        let evt = {
            tokio::select! {
                line = stdin.next_line() => Some(EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
                event = swarm.next() => {
                    info!("Unhandled Swarm Event: {:?}", event);
                    None
                },
                response = response_rcv.recv() => Some(EventType::Response(response.expect("response exists"))),
            }
        };
        ...
    }
}

struct NetHandler {
    sub: Floodsub,
    mdns: TokioMdns,
    tx: mpsc::Sender,
}