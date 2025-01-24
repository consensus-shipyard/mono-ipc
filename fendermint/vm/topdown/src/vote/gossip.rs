// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::vote::error::Error;
use crate::vote::payload::Vote;
use async_trait::async_trait;
use std::sync::Arc;

/// The gossip client communicates with the underlying gossip pub/sub network on various
/// subscribed topics. This client handles the event listening.
#[async_trait]
pub trait GossipReceiver {
    /// Attempts to poll if there are available vote. This method returns immediately.
    /// If there is no vote, it returns None
    async fn recv_vote(&mut self) -> Result<Vote, Error>;
}

#[async_trait]
pub trait GossipSender {
    async fn publish_vote(&self, vote: Vote) -> Result<(), Error>;
}

#[async_trait]
impl<S: Send + Sync + 'static + GossipSender> GossipSender for Arc<S> {
    async fn publish_vote(&self, vote: Vote) -> Result<(), Error> {
        self.as_ref().publish_vote(vote).await
    }
}
