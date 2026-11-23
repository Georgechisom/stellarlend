//! Gauge weight voting with checkpointed balance snapshots (#1576).
use soroban_sdk::{Address, Env};
use crate::storage::StorageKey;

/// Record a checkpoint of `weight` for `voter` at the current ledger.
pub fn checkpoint(env: &Env, voter: &Address, weight: i128) {
    let seq = env.ledger().sequence() as u64;
    env.storage().persistent().set(&StorageKey::GaugeCheckpoint(voter.clone(), seq), &weight);
    env.storage().persistent().set(&StorageKey::GaugeWeight(voter.clone()), &weight);
    // Accumulate total
    let total: i128 = env.storage().persistent().get(&StorageKey::GaugeTotalWeight).unwrap_or(0);
    env.storage().persistent().set(&StorageKey::GaugeTotalWeight, &total.saturating_add(weight));
}

/// Get the latest recorded weight for `voter`.
pub fn weight_of(env: &Env, voter: &Address) -> i128 {
    env.storage().persistent().get(&StorageKey::GaugeWeight(voter.clone())).unwrap_or(0)
}

/// Get total accumulated gauge weight.
pub fn total_weight(env: &Env) -> i128 {
    env.storage().persistent().get(&StorageKey::GaugeTotalWeight).unwrap_or(0)
}
