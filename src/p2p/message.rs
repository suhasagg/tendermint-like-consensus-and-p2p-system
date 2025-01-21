use serde::{Deserialize, Serialize};

/// `P2PMessage` defines the types of messages that can be exchanged
/// between nodes in this simplified Tendermint-like protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum P2PMessage {
    /// Basic handshake or peer info message. Used to announce a node's ID and address.
    PeerInfo {
        node_id: String,
        listen_addr: String,
    },
    /// A block proposal for a given round.
    Proposal {
        proposer_id: String,
        round: u64,
        block: String,
    },
    /// A prevote message for a given round/block.
    Prevote {
        voter_id: String,
        round: u64,
        block_hash: String,
    },
    /// A precommit message for a given round/block.
    Precommit {
        voter_id: String,
        round: u64,
        block_hash: String,
    },
    /// A final commit announcement for a block at a specific round.
    Commit {
        block_hash: String,
        round: u64,
    }
}

impl P2PMessage {
    /// Returns a human-readable string representing the message type.
    /// Useful for logging and debugging.
    pub fn msg_type(&self) -> &'static str {
        match self {
            P2PMessage::PeerInfo { .. } => "PeerInfo",
            P2PMessage::Proposal { .. } => "Proposal",
            P2PMessage::Prevote { .. } => "Prevote",
            P2PMessage::Precommit { .. } => "Precommit",
            P2PMessage::Commit { .. } => "Commit",
        }
    }
}

