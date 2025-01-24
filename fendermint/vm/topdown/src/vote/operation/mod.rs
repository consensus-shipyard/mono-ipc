// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

mod active;
mod paused;

use crate::vote::gossip::{GossipReceiver, GossipSender};
use crate::vote::operation::active::ActiveOperationMode;
use crate::vote::operation::paused::PausedOperationMode;
use crate::vote::store::VoteStore;
use crate::vote::VotingHandler;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum OperationMode {
    Paused = 0,
    Active = 1,
}

/// The operation state machine of voting reactor.
///
/// Active: Active publishing votes and aggregating votes normally
/// Paused: Stops voting reactor due to unknown or irrecoverable issues
///
/// State Diagram:
///   [*] --> Paused : Process start
///   Paused --> Active : Synced
///   Active --> Recovery : Checkpoints quiet
///   Recovery --> Active : Checkpoints observed
///   Active --> Paused : Stopping
///   Recovery --> Paused : Stopping
///
///   State: Recovery {
///     [*] --> SoftRecovery
///     SoftRecovery --> HardRecovery : Still no new topdown checkpoints
///     SoftRecovery --> [*] : New checkpoints
///     HardRecovery --> [*] : New checkpoints
///   }
/// TODO: Soft and Hard recovery mode to be added
pub enum OperationStateMachine<T, S, V> {
    Paused(PausedOperationMode<T, S, V>),
    Active(ActiveOperationMode<T, S, V>),
}

/// Tracks the operation mdoe metrics for the voting system
#[derive(Clone, Debug)]
pub struct OperationMetrics {
    current_mode: OperationMode,
    previous_mode: Option<OperationMode>,
}

impl<T, S, V> OperationStateMachine<T, S, V> {
    /// Always start with Paused operation mode, one needs to know the exact status from syncer.
    pub fn new(handler: VotingHandler<T, S, V>) -> OperationStateMachine<T, S, V> {
        let metrics = OperationMetrics {
            current_mode: OperationMode::Paused,
            previous_mode: None,
        };
        Self::Paused(PausedOperationMode { metrics, handler })
    }
}

impl<
        T: GossipSender + Send + Sync + 'static + Clone,
        S: GossipReceiver + Send + Sync + 'static,
        V: VoteStore + Send + Sync + 'static,
    > OperationStateMachine<T, S, V>
{
    pub async fn step(self) -> Self {
        match self {
            OperationStateMachine::Paused(p) => p.advance().await,
            OperationStateMachine::Active(p) => p.advance().await,
        }
    }
}

impl OperationMetrics {
    pub fn mode_changed(&mut self, mode: OperationMode) {
        self.previous_mode = Some(self.current_mode);
        self.current_mode = mode;
    }
}
