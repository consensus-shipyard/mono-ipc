// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use cid::Cid;
use fil_actors_runtime::actor_dispatch;
use fil_actors_runtime::actor_error;
use fil_actors_runtime::runtime::{ActorCode, Runtime};
use fil_actors_runtime::ActorError;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_shared::METHOD_CONSTRUCTOR;
use num_derive::FromPrimitive;
use std::collections::VecDeque;

#[derive(Serialize_tuple, Deserialize_tuple)]
struct State {
    blockhashes: VecDeque<Cid>,
    params: ConstructorParams,
}

#[derive(Default, Debug, Serialize_tuple, Deserialize_tuple)]
struct ConstructorParams {
    lookback_len: u64,
}

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Constructor = METHOD_CONSTRUCTOR,
    PushBlock = 2,
    LookbackLen = 3,
    BlockCID = 4,
}

pub struct Actor;

impl Actor {
    fn constructor(rt: &impl Runtime, params: ConstructorParams) -> Result<(), ActorError> {
        let state = State {
            blockhashes: VecDeque::new(),
            params,
        };
        rt.create(&state)?;
        Ok(())
    }

    fn push_block(rt: &impl Runtime, block: Cid) -> Result<(), ActorError> {
        rt.transaction(|st: &mut State, _rt| {
            st.blockhashes.push_back(block);
            if st.blockhashes.len() > st.params.lookback_len as usize {
                st.blockhashes.pop_front();
            }

            Ok(())
        })?;

        Ok(())
    }

    fn lookback_len(rt: &impl Runtime) -> Result<u64, ActorError> {
        let state: State = rt.state()?;
        Ok(state.params.lookback_len)
    }

    fn block_cid(rt: &impl Runtime, rewind: u64) -> Result<Cid, ActorError> {
        let state: State = rt.state()?;
        let block = state
            .blockhashes
            .get(state.blockhashes.len() - rewind as usize - 1)
            .ok_or_else(|| actor_error!(illegal_argument; "lookback too large"))?;

        Ok(*block)
    }
}

impl ActorCode for Actor {
    type Methods = Method;

    fn name() -> &'static str {
        "ChainMetadata"
    }

    actor_dispatch! {
        Constructor => constructor,
        PushBlock => push_block,
        LookbackLen => lookback_len,
        BlockCID => block_cid,
    }
}
