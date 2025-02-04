// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

mod operation;

use crate::sync::TopDownSyncEvent;
use crate::vote::operation::OperationStateMachine;
use crate::BlockHeight;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};

#[derive(Clone)]
pub struct VoteRecord {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// The reactor request channel buffer size
    req_channel_buffer_size: usize,
}

pub struct VoteReactorClient {
    tx: mpsc::Sender<VoteReactorRequest>,
}

pub fn start_vote_reactor(
    config: Config,
    gossip_rx: broadcast::Receiver<VoteRecord>,
    gossip_tx: mpsc::Sender<VoteRecord>,
    internal_event_listener: broadcast::Receiver<TopDownSyncEvent>,
) -> VoteReactorClient {
    let (tx, rx) = mpsc::channel(config.req_channel_buffer_size);

    tokio::spawn(async move {
        let inner = VotingHandler {
            req_rx: rx,
            gossip_rx,
            gossip_tx,
            internal_event_listener,
            config,
        };
        let mut machine = OperationStateMachine::new(inner);
        loop {
            machine = machine.step().await;
        }
    });

    VoteReactorClient { tx }
}

enum VoteReactorRequest {
    QueryOperationMode,
    QueryVotes(BlockHeight),
}

struct VotingHandler {
    /// Handles the requests targeting the vote reactor, could be querying the
    /// vote tally status and etc.
    req_rx: mpsc::Receiver<VoteReactorRequest>,
    /// Receiver from gossip pub/sub, mostly listening to incoming votes
    gossip_rx: broadcast::Receiver<VoteRecord>,
    /// Sender for gossip pub/sub, publishing new votes signed by current node
    gossip_tx: mpsc::Sender<VoteRecord>,
    /// Listens to internal events and handles the events accordingly
    internal_event_listener: broadcast::Receiver<TopDownSyncEvent>,
    config: Config,
}

impl VotingHandler {
    fn handle_request(&self, _req: VoteReactorRequest) {}

    fn record_vote(&self, _vote: VoteRecord) {}

    fn handle_event(&self, _event: TopDownSyncEvent) {}
}
