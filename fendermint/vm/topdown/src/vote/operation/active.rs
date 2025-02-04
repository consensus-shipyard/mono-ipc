// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::vote::operation::paused::PausedOperationMode;
use crate::vote::operation::{
    OperationMetrics, OperationMode, OperationModeHandler, OperationStateMachine,
};
use crate::vote::TopDownSyncEvent;
use crate::vote::VotingHandler;
use async_trait::async_trait;
use std::fmt::{Display, Formatter};
use tokio::select;

/// In active mode, we observe a steady rate of topdown checkpoint commitments on chain.
/// Our lookahead buffer is sliding continuously. As we acquire new finalised parent blocks,
/// we broadcast individual signed votes for every epoch.
pub(crate) struct ActiveOperationMode {
    pub(crate) metrics: OperationMetrics,
    pub(crate) handler: VotingHandler,
}

impl Display for ActiveOperationMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "active")
    }
}

impl ActiveOperationMode {
    fn into_paused(mut self) -> OperationStateMachine {
        self.metrics.mode_changed(OperationMode::Paused);
        OperationStateMachine::Paused(PausedOperationMode {
            metrics: self.metrics,
            handler: self.handler,
        })
    }
}

#[async_trait]
impl OperationModeHandler for ActiveOperationMode {
    async fn advance(mut self) -> OperationStateMachine {
        loop {
            select! {
                Some(req) = self.handler.req_rx.recv() => {
                    self.handler.handle_request(req);
                },
                Ok(vote) = self.handler.gossip_rx.recv() => {
                    self.handler.record_vote(vote);

                    // TODO: need to handle soft recovery transition
                },
                Ok(event) = self.handler.internal_event_listener.recv() => {
                    // top down is now syncing, pause everything
                    if matches!(event, TopDownSyncEvent::NodeSyncing) {
                        return self.into_paused();
                    }
                    self.handler.handle_event(event);
                }
            }
        }
    }
}
