/// The consensus module includes the main data structures and logic
/// needed to run a simplified Tendermint-like round-based consensus.
///
/// It consists of:
/// - A `ConsensusState` struct that holds references to shared data.
/// - The main `run_consensus_loop` function, which drives the consensus steps.
/// - Submodules like `state.rs`, `types.rs`, and `validator.rs`.

use anyhow::Result;
use std::sync::{Arc, Mutex};

use tracing::{debug, info, warn};

use crate::p2p::message::P2PMessage;
use crate::p2p::peer::{Peer, PeerManager};

pub mod state;
pub mod types;
pub mod validator;

use state::ConsensusCore;
use types::{RoundState, Step};
use validator::ValidatorSet;

/// `ConsensusState` is the primary handle that the rest of the application
/// uses to interact with the consensus engine.
///
/// It holds:
/// - A unique `node_id` for the local node
/// - The local listen address
/// - A `PeerManager` to track known peers
/// - A `ConsensusCore` that implements the internal logic
#[derive(Clone)]
pub struct ConsensusState {
    /// The unique ID of this node.
    pub node_id: String,
    /// The TCP address (host:port) on which this node listens.
    pub listen_addr: String,

    /// Manages the list of known peers.
    peer_manager: PeerManager,

    /// The core consensus logic and state.
    consensus_core: Arc<Mutex<ConsensusCore>>,
}

impl ConsensusState {
    /// Creates a new `ConsensusState` with a given `node_id` and `listen_addr`.
    ///
    /// Also initializes a `PeerManager` and a `ConsensusCore`.
    pub fn new(node_id: String, listen_addr: String) -> Self {
        let peer_manager = PeerManager::new();
        let consensus_core = ConsensusCore::new(node_id.clone(), listen_addr.clone());

        Self {
            node_id,
            listen_addr,
            peer_manager,
            consensus_core: Arc::new(Mutex::new(consensus_core)),
        }
    }

    /// Called whenever a P2P message arrives from a peer.
    ///
    /// This function delegates to more specific handlers depending on the message type.
    pub async fn process_p2p_message(&self, msg: P2PMessage) -> Result<()> {
        debug!("process_p2p_message: {:?}", msg);

        match msg {
            // A peer announces itself
            P2PMessage::PeerInfo { node_id, listen_addr } => {
                info!("Received PeerInfo from {} at {}", node_id, listen_addr);
                let peer = Peer::new(node_id, listen_addr);
                self.peer_manager.add_peer(peer);
            }
            // A new block proposal
            P2PMessage::Proposal { proposer_id, round, block } => {
                self.handle_proposal(proposer_id, round, block).await?;
            }
            // A prevote
            P2PMessage::Prevote { voter_id, round, block_hash } => {
                self.handle_prevote(voter_id, round, block_hash).await?;
            }
            // A precommit
            P2PMessage::Precommit { voter_id, round, block_hash } => {
                self.handle_precommit(voter_id, round, block_hash).await?;
            }
            // A commit
            P2PMessage::Commit { block_hash, round } => {
                self.handle_commit(block_hash, round).await?;
            }
        }

        Ok(())
    }

    // ----- Handlers for each message type -----

    /// Handle a `Proposal` message from a peer (including ourselves).
    async fn handle_proposal(&self, proposer_id: String, round: u64, block: String) -> Result<()> {
        let mut core = self.consensus_core.lock().unwrap();
        core.on_proposal(proposer_id, round, block)
    }

    /// Handle a `Prevote` message from a peer (including ourselves).
    async fn handle_prevote(&self, voter_id: String, round: u64, block_hash: String) -> Result<()> {
        let mut core = self.consensus_core.lock().unwrap();
        core.on_prevote(voter_id, round, block_hash)
    }

    /// Handle a `Precommit` message from a peer (including ourselves).
    async fn handle_precommit(&self, voter_id: String, round: u64, block_hash: String) -> Result<()> {
        let mut core = self.consensus_core.lock().unwrap();
        core.on_precommit(voter_id, round, block_hash)
    }

    /// Handle a `Commit` message from a peer (including ourselves).
    async fn handle_commit(&self, block_hash: String, round: u64) -> Result<()> {
        let mut core = self.consensus_core.lock().unwrap();
        core.on_commit(block_hash, round)
    }

    // ----- Utilities -----

    /// Broadcasts a message to all known peers.
    ///
    /// This function looks up all peers in the `PeerManager` and,
    /// for each peer, spawns a task to send the message via `send_message`.
    pub async fn broadcast_message(&self, msg: &P2PMessage) {
        use crate::p2p::transport::send_message;
        let peers = self.peer_manager.get_all_peers();
        for peer in peers {
            let addr = match peer.listen_addr.parse() {
                Ok(a) => a,
                Err(e) => {
                    warn!("Invalid peer address {}: {:?}", peer.listen_addr, e);
                    continue;
                }
            };
            let msg_clone = msg.clone();
            tokio::spawn(async move {
                if let Err(e) = send_message(addr, &msg_clone).await {
                    warn!("Failed to send {} to {}: {:?}", msg_clone.msg_type(), addr, e);
                }
            });
        }
    }
}

/// The main logic loop for the consensus protocol.
///
/// In real Tendermint, there is a complex interplay of
/// timeouts, round increments, and the Propose/Prevote/Precommit steps.
/// This simplified loop just starts a new round (with a new block) every 10 seconds.
///
/// # Arguments
///
/// * `cs` - The consensus state to operate on.
pub async fn run_consensus_loop(cs: ConsensusState) {
    use tokio::time::{sleep, Duration};
    loop {
        sleep(Duration::from_secs(10)).await;

        let new_block = format!("block-{}", uuid::Uuid::new_v4());
        info!("Proposing a new block: {}", new_block);

        let mut core = cs.consensus_core.lock().unwrap();
        core.start_new_round(new_block);
    }
}

