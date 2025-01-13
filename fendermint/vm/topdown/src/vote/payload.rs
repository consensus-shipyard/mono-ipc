// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::observation::{CertifiedObservation, Observation};
use crate::vote::Weight;
use crate::BlockHeight;
use anyhow::anyhow;
use fendermint_crypto::secp::RecoverableECDSASignature;
use fendermint_vm_genesis::ValidatorKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type PowerTable = HashMap<ValidatorKey, Weight>;
pub type PowerUpdates = Vec<(ValidatorKey, Weight)>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct VoteTallyState {
    pub last_finalized_height: BlockHeight,
    pub quorum_threshold: Weight,
    pub power_table: PowerTable,
}

/// The different versions of vote casted in topdown gossip pub-sub channel
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum Vote {
    V1 {
        validator: ValidatorKey,
        payload: CertifiedObservation,
    },
}

impl Vote {
    pub fn v1(validator_key: ValidatorKey, obs: CertifiedObservation) -> Self {
        Self::V1 {
            validator: validator_key,
            payload: obs,
        }
    }

    pub fn v1_checked(obs: CertifiedObservation) -> anyhow::Result<Self> {
        Ok(Self::V1 {
            validator: obs.ensure_valid()?,
            payload: obs,
        })
    }

    pub fn voter(&self) -> ValidatorKey {
        match self {
            Self::V1 { validator, .. } => validator.clone(),
        }
    }

    pub fn observation(&self) -> &Observation {
        match self {
            Self::V1 { payload, .. } => payload.observation(),
        }
    }

    pub fn observation_signature(&self) -> &RecoverableECDSASignature {
        match self {
            Self::V1 { payload, .. } => payload.observation_signature(),
        }
    }
}

impl TryFrom<&[u8]> for Vote {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let version = bytes[0];

        if version == 0 {
            return Self::v1_checked(CertifiedObservation::try_from(&bytes[1..])?);
        }

        Err(anyhow!("invalid vote version"))
    }
}
