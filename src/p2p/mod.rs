/// The P2P module contains functionality for peer management, message definitions,
/// and transport (TCP) logic. It exposes high-level functions for starting
/// listeners and making outbound connections.

use anyhow::Result;
use std::net::SocketAddr;

use crate::consensus::ConsensusState;

pub mod message;
pub mod peer;
pub mod transport;

use peer::{Peer};
use transport::{accept_loop, connect_to_peer};

/// Start listening for inbound connections using a TCP listener.
/// Spawns an `accept_loop` to handle connections as they arrive.
///
/// # Arguments
///
/// * `cs` - The shared consensus state.
/// * `listen_addr` - The address (host:port) to bind for inbound connections.
pub async fn start_listening(cs: ConsensusState, listen_addr: &str) -> Result<()> {
    let socket_addr: SocketAddr = listen_addr.parse()?;
    accept_loop(cs, socket_addr).await
}

/// Attempt outbound connections to a list of known peer addresses.
///
/// # Arguments
///
/// * `cs` - The shared consensus state.
/// * `peers` - A list of string addresses of known peers (host:port).
pub async fn start_outbound_connections(cs: ConsensusState, peers: Vec<&str>) {
    for peer_addr in peers {
        let addr: SocketAddr = match peer_addr.parse() {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Invalid peer address {}: {:?}", peer_addr, e);
                continue;
            }
        };

        // Spawn a task to connect to this peer
        tokio::spawn({
            let cs_clone = cs.clone();
            async move {
                if let Err(e) = connect_to_peer(cs_clone, addr).await {
                    eprintln!("Failed to connect to {}: {:?}", addr, e);
                }
            }
        });
    }
}

