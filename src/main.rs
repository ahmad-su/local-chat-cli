extern crate libp2p;
use std::error::Error;

use libp2p::{identity, PeerId};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    println!("Local peer ID: {}", local_peer_id);

    Ok(())
}
