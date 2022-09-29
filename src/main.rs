extern crate libp2p;
use std::error::Error;

use libp2p::{
    identity, PeerId, 
    ping::{
        Ping, PingConfig
    }, Swarm, Multiaddr, futures::StreamExt, swarm::SwarmEvent,
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>>{
    //Generate and assign key as peer id
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    println!("Local peer ID: {}", local_peer_id);

    //Construct transport (based on TCP)
    //Just as its name, the transport will be used as a transport to deliver package
    let transport = libp2p::development_transport(local_key).await?;

    //Create Network behaviour. 
    //this will define what kind of package to send
    //This time we implement this behaviour to be a ping package.
    //first step to make sure p2p network is working
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    //Now create swarm to connect between behaviour and transport.
    //it also listens to port in which the transport go through.
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    //Now tell the swarm to listen on all random OS-assigned port
    //now it's ready to send and receive ping
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    
    //let's try giving it an address to ping on cli.
    //format : cargo run [MultiAddr]
    //example: cargo run /ip4/127.0.0.1/tcp/12321
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }

    //you have to run this app in 2 separate terminal, 
    //first as a target, run without arguments, and the second as the pinger
    //ping using the addr given in terminal 1
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }
}
