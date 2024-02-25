mod args;
mod message;
mod peer;

use std::time::Duration;

use clap::Parser;

use self::args::Args;
use self::peer::Peer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    let peer = Peer::new(args.port);

    let conns: Vec<&str> = args.connect.split(",").collect();
    for addr in conns.iter() {
        peer.gossip(addr.to_string(), Duration::from_secs(args.period.into()));
    }

    peer.listen().await;
    Ok(())
}

