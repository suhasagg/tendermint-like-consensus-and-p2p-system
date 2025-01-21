/// A simple placeholder for storing validator identities.
/// Real Tendermint uses dynamic validator sets, changes, staking, etc.

/// Represents a set of validators, each with an ID.
/// In real usage, these IDs would be public keys.
#[derive(Debug)]
pub struct ValidatorSet {
    /// List of validators (IDs as strings).
    pub validators: Vec<String>,
}

impl ValidatorSet {
    /// Constructs a validator set from a given vector of IDs.
    pub fn new_simple(validators: Vec<String>) -> Self {
        Self { validators }
    }

    /// Returns the total number of validators in the set.
    pub fn len(&self) -> usize {
        self.validators.len()
    }

    /// Checks if the set contains a validator with the specified `id`.
    pub fn contains(&self, id: &str) -> bool {
        self.validators.contains(&id.to_string())
    }
}

