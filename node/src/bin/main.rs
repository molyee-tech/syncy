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

    let transport = TokioTcpConfig::new()
        .upgrade(upversion)
        .authenticate(creds)
        .multiplex(mplex)
        .boxed();
    let mdns = TokioMdns::new().expect("can create mdns");
    let sub = Floodsub::new(PEER_ID.clone());
    let mut handler = NetHandler { sub, mdns, tx };
    handler.sub.subscribe(TOPIC.clone());

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
        line = stdin.select_next_some() => handle_input_line(&mut swarm.behaviour_mut().kademlia, line.expect("Stdin not to close")),
        event = swarm.select_next_some() => match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening in {:?}", address);
            },
            SwarmEvent::Behaviour(Event::Mdns(MdnsEvent::Discovered(list))) => {
                for (peer_id, multiaddr) in list {
                    swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr);
                }
            }
            SwarmEvent::Behaviour(Event::Kademlia(KademliaEvent::OutboundQueryCompleted { result, ..})) => {
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
            SwarmEvent::Behaviour(Event::Identify(event)) => {
                println!("identify: {:?}", event);
            }
            SwarmEvent::Behaviour(Event::Gossipsub(GossipsubEvent::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            })) => {
                println!(
                    "Got message: {} with id: {} from peer: {:?}",
                    String::from_utf8_lossy(&message.data),
                    id,
                    peer_id
                )
            }
            SwarmEvent::Behaviour(Event::Ping(event)) => {
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
            _ => {}
        }
    }
}

fn handle_input_line(kademlia: &mut Kademlia<MemoryStore>, line: String) {
    let mut args = line.split(' ');

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

struct NetHandler {
    sub: Floodsub,
    mdns: TokioMdns,
    tx: mpsc::Sender,
}