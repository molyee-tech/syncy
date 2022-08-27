
pub enum Event {
    Kademlia(KademliaEvent),
    Mdns(MdnsEvent),
}

impl From<KademliaEvent> for Event {
    fn from(event: KademliaEvent) -> Self {
        Event::Kademlia(event)
    }
}

impl From<MdnsEvent> for Event {
    fn from(event: MdnsEvent) -> Self {
        Event::Mdns(event)
    }
}
