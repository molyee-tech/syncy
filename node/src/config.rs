pub struct Config {
    peer_id: PeerId,
    peers: Peers,
    metrics: Metrics,
}

pub struct Peers(Vec<Peer>);

pub struct Peer(String);

pub struct Metrics {
    connected: u32,
}

