use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

mod p2p;
mod consensus;

use p2p::{start_listening, start_outbound_connections};
use consensus::{ConsensusState, run_consensus_loop};

/// Main entry point of our Tendermint-like node.
///
/// This sets up logging, initializes the consensus state,
/// spawns tasks for P2P inbound/outbound connections,
/// and runs the main consensus loop.
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging (tracing)
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Generate a unique node ID for demonstration:
    let node_id = Uuid::new_v4().to_string();
    // The TCP address on which this node will listen for inbound connections
    let listen_addr = "127.0.0.1:7000";

    // Known peers. In a real system, these might come from config files, seeds,
    // or peer discovery. For demonstration, keep it empty or specify known addresses.
    let known_peers = vec![
        // e.g. "127.0.0.1:7001", "127.0.0.1:7002", ...
    ];

    // Create the main consensus state object
    let consensus_state = ConsensusState::new(node_id.clone(), listen_addr.to_string());

    info!("Node {} starting up on {}...", node_id, listen_addr);

    // Spawn a task to listen for inbound P2P connections
    tokio::spawn({
        let cs = consensus_state.clone();
        async move {
            if let Err(e) = start_listening(cs, listen_addr).await {
                eprintln!("P2P listener error: {:?}", e);
            }
        }
    });

    // Spawn a task to attempt outbound connections to known peers
    tokio::spawn({
        let cs = consensus_state.clone();
        async move {
            start_outbound_connections(cs, known_peers).await;
        }
    });

    // Spawn the main consensus loop
    tokio::spawn({
        let cs = consensus_state.clone();
        async move {
            run_consensus_loop(cs).await;
        }
    });

    // Keep the main function alive
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}

