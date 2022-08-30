mod error;
mod behavior;
mod event;
mod storage;

pub use error::{Error, Result};

use libp2p::identity::Keypair;
use libp2p::PeerId;
use libp2p::floodsub::Topic;
use once_cell::sync::Lazy;

pub static KEYS: Lazy<Keypair> = Lazy::new(|| Keypair::generate_ed25519());
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
pub static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("messages"));