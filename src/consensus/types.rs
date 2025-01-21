use std::collections::HashMap;

/// The consensus steps in a simplified Tendermint-like round.
#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    /// Propose: A node proposes a new block.
    Propose,
    /// Prevote: Nodes broadcast votes after receiving a proposal.
    Prevote,
    /// Precommit: After prevotes, nodes broadcast precommits if certain thresholds are met.
    Precommit,
    /// Commit: Once enough precommits are received, the block is considered committed.
    Commit,
}

/// Holds metadata for the current round, including which step we're on,
/// the proposed block, and votes (prevotes/precommits).
#[derive(Debug, Default)]
pub struct RoundState {
    /// The round number (increases whenever there's a new attempt to agree on a block).
    pub round: u64,
    /// The step within the round (Propose, Prevote, Precommit, or Commit).
    pub step: Step,
    /// The proposed block for this round (if any).
    pub proposal: Option<String>,
    /// If a block is locked, it means we've decided to proceed with that block
    /// unless a higher round decides otherwise (Tendermint's "lock" mechanism).
    pub locked_block_hash: Option<String>,
    /// Collection of prevotes from validators (maps voter ID to the block hash they voted for).
    pub prevotes: HashMap<String, String>,
    /// Collection of precommits from validators (maps voter ID to the block hash they voted for).
    pub precommits: HashMap<String, String>,
}

impl RoundState {
    /// Constructs a new `RoundState` with round = 0, step = Propose, and empty votes.
    pub fn new() -> Self {
        Self {
            round: 0,
            step: Step::Propose,
            proposal: None,
            locked_block_hash: None,
            prevotes: HashMap::new(),
            precommits: HashMap::new(),
        }
    }
}

/// Configuration parameters for the consensus protocol, e.g., how many votes are needed, timeouts, etc.
#[derive(Debug)]
pub struct ConsensusParams {
    /// The fraction of validators needed to reach a quorum (e.g., 2/3).
    pub quorum_threshold: f32,
}

impl Default for ConsensusParams {
    fn default() -> Self {
        Self {
            quorum_threshold: 0.67,
        }
    }
}
