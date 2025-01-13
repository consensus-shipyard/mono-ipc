// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::vote::gossip::{GossipReceiver, GossipSender};
use crate::vote::operation::active::ActiveOperationMode;
use crate::vote::operation::{OperationMetrics, OperationMode, OperationStateMachine};
use crate::vote::store::VoteStore;
use crate::vote::TopDownSyncEvent;
use crate::vote::VotingHandler;
use std::fmt::{Display, Formatter};
use tokio::select;

/// The paused operation mode handler.
///
/// Paused mode engages when we’re catching up with the subnet chain
/// (usually after a process interruption, restart, or first start).
/// Therefore, we still don’t know what the last committed topdown checkpoint is,
/// so we refrain from watching the parent chain, and from gossiping
/// any certified observations until we switch to active mode.
pub(crate) struct PausedOperationMode<T, S, V> {
    pub metrics: OperationMetrics,
    pub(crate) handler: VotingHandler<T, S, V>,
}

impl<T, S, V> Display for PausedOperationMode<T, S, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PAUSED")
    }
}

impl<T, S, V> PausedOperationMode<T, S, V> {
    fn into_active(mut self) -> OperationStateMachine<T, S, V> {
        self.metrics.mode_changed(OperationMode::Active);
        OperationStateMachine::Active(ActiveOperationMode {
            metrics: self.metrics,
            handler: self.handler,
        })
    }
}

impl<
        T: GossipSender + Send + Sync + 'static + Clone,
        S: GossipReceiver + Send + Sync + 'static,
        V: VoteStore + Send + Sync + 'static,
    > PausedOperationMode<T, S, V>
{
    pub(crate) async fn advance(mut self) -> OperationStateMachine<T, S, V> {
        loop {
            select! {
                Some(req) = self.handler.req_rx.recv() => {
                    self.handler.handle_request(req, &self.metrics);
                },
                Ok(vote) = self.handler.gossip_rx.recv_vote() => {
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
