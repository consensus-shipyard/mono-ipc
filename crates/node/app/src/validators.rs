// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use anyhow::{anyhow, Ok, Result};
use fendermint_crypto::PublicKey;
use fendermint_vm_interpreter::fvm::state::ipc::GatewayCaller;
use fendermint_vm_interpreter::fvm::state::FvmExecState;
use std::collections::HashMap;

use tendermint::account::Id as TendermintId;
use tendermint::PublicKey as TendermintPubKey;

use fvm_ipld_blockstore::Blockstore;

#[derive(Clone)]
// Tracks the validator ID from Tendermint to their corresponding public key.
pub(crate) struct ValidatorCache {
    map: HashMap<TendermintId, PublicKey>,
}

impl ValidatorCache {
    pub fn new_from_state<SS>(state: &mut FvmExecState<SS>) -> Result<Self>
    where
        SS: Blockstore + Clone + 'static,
    {
        let gateway = GatewayCaller::default();
        let (_, validators) = gateway.current_power_table(state)?;

        let map = validators
            .iter()
            .map(|v| {
                let tendermint_pub_key: TendermintPubKey =
                    TendermintPubKey::try_from(v.public_key.clone())?;
                let id = TendermintId::from(tendermint_pub_key);
                let key = *v.public_key.public_key();
                Ok((id, key))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Self { map })
    }

    pub fn get_validator(&self, id: &tendermint::account::Id) -> Result<PublicKey> {
        self.map
            .get(id)
            .cloned()
            .ok_or_else(|| anyhow!("validator not found"))
    }
}
