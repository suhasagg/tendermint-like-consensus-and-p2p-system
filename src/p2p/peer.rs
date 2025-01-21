use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::message::P2PMessage;

/// Represents a peer in the network, storing an ID (often a public key or unique string)
/// and the address at which the peer listens for inbound connections.
#[derive(Debug, Clone)]
pub struct Peer {
    /// A unique identifier for the peer.
    pub id: String,
    /// The TCP address (host:port) on which this peer listens.
    pub listen_addr: String,
}

impl Peer {
    /// Creates a new peer with the given ID and address.
    pub fn new(id: String, addr: String) -> Self {
        Self {
            id,
            listen_addr: addr,
        }
    }
}

/// `PeerManager` holds a collection of known peers (by ID).
///
/// In a real system, you'd also track connection states,
/// availability, and more advanced metadata about each peer.
#[derive(Clone)]
pub struct PeerManager {
    /// A thread-safe map of peer_id -> Peer
    inner: Arc<Mutex<HashMap<String, Peer>>>,
}

impl PeerManager {
    /// Constructs a new, empty peer manager.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Inserts or updates a peer in the internal map.
    ///
    /// # Arguments
    /// * `peer` - The peer to insert or update.
    pub fn add_peer(&self, peer: Peer) {
        let mut map = self.inner.lock().unwrap();
        map.insert(peer.id.clone(), peer);
    }

    /// Retrieves a **copy** of all currently known peers.
    ///
    /// Returns a vector of `Peer` structs.
    pub fn get_all_peers(&self) -> Vec<Peer> {
        let map = self.inner.lock().unwrap();
        map.values().cloned().collect()
    }
}

