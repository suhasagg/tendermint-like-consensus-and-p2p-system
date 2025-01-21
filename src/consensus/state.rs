/// `ConsensusCore` implements the low-level logic for each consensus round.
/// It stores the current round state, a validator set, and methods to respond
/// to inbound messages (proposal, prevote, precommit, commit).

use anyhow::Result;
use tracing::{debug, info};

use super::types::{RoundState, Step, ConsensusParams};
use super::validator::ValidatorSet;

/// Core structure holding the local node's consensus-related data.
#[derive(Debug)]
pub struct ConsensusCore {
    /// The ID of the local node (often a validator key or similar).
    pub node_id: String,
    /// The address on which this node listens for inbound connections.
    pub listen_addr: String,

    /// The set of validators participating in consensus (simplified here).
    pub validators: ValidatorSet,

    /// The current round state (round number, step, locked block, etc.).
    pub round_state: RoundState,

    /// Configuration parameters, e.g., the threshold for quorum.
    pub params: ConsensusParams,
}

impl ConsensusCore {
    /// Constructs a new `ConsensusCore` object with a simple validator set
    /// containing just our local node (for demonstration).
    pub fn new(node_id: String, listen_addr: String) -> Self {
        let validators = ValidatorSet::new_simple(vec![node_id.clone()]);
        let round_state = RoundState::default();
        let params = ConsensusParams::default();

        Self {
            node_id,
            listen_addr,
            validators,
            round_state,
            params,
        }
    }

    /// Triggers a new consensus round, typically by the local node acting
    /// as the proposer. Sets the step to `Propose`, updates the round number,
    /// and registers the proposed block.
    ///
    /// # Arguments
    ///
    /// * `block` - A string representing the newly proposed block.
    pub fn start_new_round(&mut self, block: String) {
        let new_round = self.round_state.round + 1;
        self.round_state.round = new_round;
        self.round_state.step = Step::Propose;
        self.round_state.proposal = Some(block);
        self.round_state.locked_block_hash = None;
        self.round_state.prevotes.clear();
        self.round_state.precommits.clear();

        info!("Starting new round: {}", new_round);
        info!("Proposed block: {:?}", self.round_state.proposal);
    }

    // ----- Event Handlers -----

    /// Called when we receive a `Proposal` message from some node.
    ///
    /// # Arguments
    ///
    /// * `proposer_id` - ID of the node that proposed the block.
    /// * `round` - The round number of the proposal.
    /// * `block` - The proposed block contents.
    pub fn on_proposal(&mut self, proposer_id: String, round: u64, block: String) -> Result<()> {
        debug!("on_proposal: from={} round={} block={}", proposer_id, round, block);

        // If it's an older round, ignore.
        if round < self.round_state.round {
            return Ok(());
        }

        // If we're not in the Propose step, we might be out of sync; just ignore in this demo.
        if self.round_state.step != Step::Propose {
            return Ok(());
        }

        // Accept the proposal (in real logic, you'd validate the block, etc.)
        self.round_state.proposal = Some(block);
        Ok(())
    }

    /// Called when we receive a `Prevote` message.
    pub fn on_prevote(&mut self, voter_id: String, round: u64, block_hash: String) -> Result<()> {
        debug!("on_prevote: from={} round={} block_hash={}", voter_id, round, block_hash);

        // Store the prevote in the round state. In real Tendermint,
        // you would check if it matches your proposed block, etc.
        self.round_state.prevotes.insert(voter_id, block_hash);

        Ok(())
    }

    /// Called when we receive a `Precommit` message.
    pub fn on_precommit(&mut self, voter_id: String, round: u64, block_hash: String) -> Result<()> {
        debug!("on_precommit: from={} round={} block_hash={}", voter_id, round, block_hash);

        self.round_state.precommits.insert(voter_id, block_hash);
        Ok(())
    }

    /// Called when we receive a `Commit` message, signifying the network
    /// has committed a block at a given round.
    pub fn on_commit(&mut self, block_hash: String, round: u64) -> Result<()> {
        info!("on_commit: block_hash={} round={}", block_hash, round);
        // In real code, you'd finalize the block, store it, etc.
        Ok(())
    }
}

