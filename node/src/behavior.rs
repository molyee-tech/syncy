use crate::event::Event;
use libp2p::NetworkBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
pub struct Behaviour {
    kademlia: Kademlia<MemoryStore>,
    mdns: Mdns,
}
