use crate::event::Event;
use libp2p::NetworkBehaviour;
use libp2p::indentify::Identify;
use libp2p::ping::{Ping, PingConfig};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
pub struct Behaviour {
    kademlia: Kademlia<MemoryStore>,
    mdns: Mdns,
    gossipsub: Gossipsub,
    identify: Identify,
    ping: Ping,
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

        let mdns = TokioMdns::new().expect("can create mdns");
        let sub = Floodsub::new(PEER_ID.clone());
        let mut handler = NetHandler { sub, mdns, tx };
        handler.sub.subscribe(TOPIC.clone());

        let store = MemoryStore::new(self.peer);
        let kademlia = Kademlia::new(self.peer, store);
        let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
        let gossip_conf = GossipsubConfigBuilder::default()
            .max_transmit_size(262144)
            .build()
            .expect("valid config");
        let gossipsub = Gossipsub::new(MessageAuthenticity::Signed(self.key), gossip_conf)
            .except("valid gossip config");
        let identify = Identify::new(IdentifyConfig::new("/ipfs/0.1.0".into(), self.key.into()));
        let ping = Ping::new(PingConfig::new());
        let behaviour = Behaviour { kademlia, mdns, gossipsub, identify, ping };
        println!("Subscribing to {:?}", gossipsub_topic);
        behavior.gossipsub.subscribe(&gossipsub_topic).unwrap();
    }

    pub fn handle_input(&mut self, input: &str) {
        let mut args = line.split(' ');
        let mut kademlia = self.kademlia;
        match args.next() {
            Some("GET") => {
                let key = {
                    match args.next() {
                        Some(key) => Key::new(&key),
                        None => {
                            eprintln!("Expected key");
                            return;
                        }
                    }
                };
                kademlia.get_record(key, Quorum::One);
            }
            Some("GET_PROVIDERS") => {
                let key = {
                    match args.next() {
                        Some(key) => Key::new(&key),
                        None => {
                            eprintln!("Expected key");
                            return;
                        }
                    }
                };
                kademlia.get_providers(key);
            }
            Some("PUT") => {
                let key = {
                    match args.next() {
                        Some(key) => Key::new(&key),
                        None => {
                            eprintln!("Expected key");
                            return;
                        }
                    }
                };
                let value = {
                    match args.next() {
                        Some(value) => value.as_bytes().to_vec(),
                        None => {
                            eprintln!("Expected value");
                            return;
                        }
                    }
                };
                let record = Record {
                    key,
                    value,
                    publisher: None,
                    expires: None,
                };
                kademlia
                    .put_record(record, Quorum::One)
                    .expect("Failed to store record locally.");
            }
            Some("PUT_PROVIDER") => {
                let key = {
                    match args.next() {
                        Some(key) => Key::new(&key),
                        None => {
                            eprintln!("Expected key");
                            return;
                        }
                    }
                };

                kademlia
                    .start_providing(key)
                    .expect("Failed to start providing key");
            }
            _ => {
                eprintln!("expected GET, GET_PROVIDERS, PUT or PUT_PROVIDER");
            }
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Mdns(MdnsEvent::Discovered(list)) => {
                for (peer_id, multiaddr) in list {
                    swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr);
                }
            }
            Event::Kademlia(KademliaEvent::OutboundQueryCompleted { result, ..}) => {
                match result {
                    QueryResult::GetProviders(Ok(ok)) => {
                        for peer in ok.providers {
                            println!(
                                "Peer {:?} provides key {:?}",
                                peer,
                                std::str::from_utf8(ok.key.as_ref()).unwrap()
                            );
                        }
                    }
                    QueryResult::GetProviders(Err(err)) => {
                        eprintln!("Failed to get providers: {:?}", err);
                    }
                    QueryResult::GetRecord(Ok(ok)) => {
                        for PeerRecord {
                            record: Record { key, value, .. },
                            ..
                        } in ok.records
                        {
                            println!(
                                "Got record {:?} {:?}",
                                std::str::from_utf8(key.as_ref()).unwrap(),
                                std::str::from_utf8(&value).unwrap(),
                            );
                        }
                    }
                    QueryResult::GetRecord(Err(err)) => {
                        eprintln!("Failed to get record: {:?}", err);
                    }
                    QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                        println!(
                            "Successfully put record {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap()
                        );
                    }
                    QueryResult::PutRecord(Err(err)) => {
                        eprintln!("Failed to put record: {:?}", err);
                    }
                    QueryResult::StartProviding(Ok(AddProviderOk { key })) => {
                        println!(
                            "Successfully put provider record {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap()
                        );
                    }
                    QueryResult::StartProviding(Err(err)) => {
                        eprintln!("Failed to put provider record: {:?}", err);
                    }
                    _ => {}
                }
            }
            Event::Identify(event) => {
                println!("identify: {:?}", event);
            }
            Event::Gossipsub(GossipsubEvent::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            }) => {
                println!(
                    "Got message: {} with id: {} from peer: {:?}",
                    String::from_utf8_lossy(&message.data),
                    id,
                    peer_id
                )
            }
            Event::Ping(event) => {
                match event {
                    ping::Event {
                        peer,
                        result: Result::Ok(ping::Success::Ping { rtt }),
                    } => {
                        println!(
                            "ping: rtt to {} is {} ms",
                            peer.to_base58(),
                            rtt.as_millis()
                        );
                    }
                    ping::Event {
                        peer,
                        result: Result::Ok(ping::Success::Pong),
                    } => {
                        println!("ping: pong from {}", peer.to_base58());
                    }
                    ping::Event {
                        peer,
                        result: Result::Err(ping::Failure::Timeout),
                    } => {
                        println!("ping: timeout to {}", peer.to_base58());
                    }
                    ping::Event {
                        peer,
                        result: Result::Err(ping::Failure::Unsupported),
                    } => {
                        println!("ping: {} does not support ping protocol", peer.to_base58());
                    }
                    ping::Event {
                        peer,
                        result: Result::Err(ping::Failure::Other { error }),
                    } => {
                        println!("ping: ping::Failure with {}: {}", peer.to_base58(), error);
                    }
                }
            }
            _ => ()
        }
    }
}