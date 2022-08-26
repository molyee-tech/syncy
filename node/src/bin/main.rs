use syncy_node::Result;
use tokio::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let (tx, rx) = mpsc::unbounded_channel();
    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&KEYS)
        .expect("can create auth keys");
}