// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::sync::TopDownSyncEvent;
use crate::vote::operation::active::ActiveOperationMode;
use crate::vote::operation::{
    OperationMetrics, OperationMode, OperationModeHandler, OperationStateMachine,
};
use crate::vote::VotingHandler;
use async_trait::async_trait;
use std::fmt::{Display, Formatter};
use tokio::select;

/// The paused operation mode handler.
///
/// Paused mode engages when we’re catching up with the subnet chain
/// (usually after a process interruption, restart, or first start).
/// Therefore, we still don’t know what the last committed topdown checkpoint is,
/// so we refrain from watching the parent chain, and from gossiping
/// any certified observations until we switch to active mode.
pub(crate) struct PausedOperationMode {
    pub(crate) metrics: OperationMetrics,
    pub(crate) handler: VotingHandler,
}

impl Display for PausedOperationMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "paused")
    }
}

impl PausedOperationMode {
    fn into_active(mut self) -> OperationStateMachine {
        self.metrics.mode_changed(OperationMode::Active);
        OperationStateMachine::Active(ActiveOperationMode {
            metrics: self.metrics,
            handler: self.handler,
        })
    }
}

#[async_trait]
impl OperationModeHandler for PausedOperationMode {
    async fn advance(mut self) -> OperationStateMachine {
        loop {
            select! {
                Some(req) = self.handler.req_rx.recv() => {
                    self.handler.handle_request(req);
                },
                Ok(vote) = self.handler.gossip_rx.recv() => {
                    self.handler.record_vote(vote);
                },
                Ok(event) = self.handler.internal_event_listener.recv() => {
                    // top down is still syncing, pause everything
                    if matches!(event, TopDownSyncEvent::NodeSyncing) {
                        continue;
                    }
                    self.handler.handle_event(event);
                    return self.into_active();
                }
            }
        }
    }
}
