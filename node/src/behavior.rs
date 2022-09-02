use crate::event::Event;
use libp2p::NetworkBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
pub struct Behaviour {
    kademlia: Kademlia<MemoryStore>,
    mdns: Mdns,
    gossipsub: Gossipsub,
    identify: Identify,
    ping: ping::Behaviour,
}

pub struct Builder {
    peer: PeerId,
    key: PublicKey,
}

impl Builder {
    pub fn new(peer: PeerId, key: PublicKey) -> Self {
        Self { peer, key }
    }

    pub fn build(self) -> Behaviour {
        let store = MemoryStore::new(self.peer);
        let kademlia = Kademlia::new(self.peer, store);
        let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
        let gossip_conf = GossipsubConfigBuilder::default()
            .max_transmit_size(262144)
            .build()
            .expect("valid config");
        let gossipsub = Gossipsub::new(MessageAuthenticity::Signed(self.key), gossip_conf)
            .except("valid gossip config");
        let identify = Identify::new(IdentifyConfig::new("/ipfs/0.1.0".into(), local_key.into()));
        let ping = ping::Behaviour::new(ping::Config::new());
        let behaviour = Behaviour { kademlia, mdns, gossipsub, identify, ping };
    }
}