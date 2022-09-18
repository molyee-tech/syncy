use libp2p::kademlia::KademliaEvent;
use libp2p::mdns::MdnsEvent;
use libp2p::gossip::GossipsubEvent;
use libp2p::identify::IdentifyEvent;
use libp2p::ping::PingEvent;

pub enum Event {
    Kademlia(KademliaEvent),
    Mdns(MdnsEvent),
    Gossipsub(GossipsubEvent),
    Identify(IdentifyEvent),
    Ping(PingEvent),
}

impl From<KademliaEvent> for Event {
    fn from(event: KademliaEvent) -> Self {
        Self::Kademlia(event)
    }
}

impl From<MdnsEvent> for Event {
    fn from(event: MdnsEvent) -> Self {
        Self::Mdns(event)
    }
}

impl From<GossipsubEvent> for Event {
    fn from(event: GossipsubEvent) -> Self {
        Self::Gossipsub(event)
    }
}

impl From<IdentifyEvent> for Event {
    fn from(event: IdentifyEvent) -> Self {
        Self::Identify(event)
    }
}

impl From<PingEvent> for Event {
    fn from(event: PingEvent) -> Self {
        Self::Ping(event)
    }
}