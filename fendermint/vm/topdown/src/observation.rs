// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::syncer::error::Error;
use crate::syncer::store::ParentViewStore;
use crate::{BlockHash, BlockHeight, Bytes, Checkpoint};
use anyhow::anyhow;
use arbitrary::Arbitrary;
use cid::Cid;
use fendermint_crypto::secp::RecoverableECDSASignature;
use fendermint_crypto::SecretKey;
use fendermint_vm_genesis::ValidatorKey;
use fvm_ipld_encoding::DAG_CBOR;
use multihash::Code;
use multihash::MultihashDigest;
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::fmt::{Display, Formatter};

use crate::syncer::payload::ParentBlockView;

/// Default topdown observation height range
const DEFAULT_MAX_OBSERVATION_RANGE: BlockHeight = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationConfig {
    /// The max number of blocks one should make the topdown observation from the previous
    /// committed checkpoint
    pub max_observation_range: Option<BlockHeight>,
}

/// The content that validators gossip among each other.
#[derive(Serialize, Deserialize, Hash, Debug, Clone, Eq, PartialEq, Arbitrary)]
pub struct Observation {
    pub(crate) parent_subnet_height: u64,
    /// The hash of the chain unit at that height. Usually a block hash, but could
    /// be another entity (e.g. tipset CID), depending on the parent chain
    /// and our interface to it. For example, if the parent is a Filecoin network,
    /// this would be a tipset CID coerced into a block hash if queried through
    /// the Eth API, or the tipset CID as-is if accessed through the Filecoin API.
    pub(crate) parent_subnet_hash: Bytes,
    /// A rolling/cumulative commitment to topdown effects since the beginning of
    /// time, including the ones in this block.
    pub(crate) cumulative_effects_comm: Bytes,
}

/// A self-certified observation made by a validator.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct CertifiedObservation {
    observation: Observation,
    /// The signature for the observation only
    observation_signature: RecoverableECDSASignature,
    /// The hash of the subnet's last committed block when this observation was made.
    /// Used to discard stale observations that are, e.g. replayed by an attacker
    /// at a later time. Also used to detect nodes that might be wrongly gossiping
    /// whilst being out of sync.
    certified_at: BlockHeight,
    /// A "recoverable" ECDSA signature with the validator's secp256k1 private key over the
    /// CID of the DAG-CBOR encoded observation using a BLAKE2b-256 multihash.
    signature: RecoverableECDSASignature,
}

/// Check in the store to see if there is a new observation available.
/// Caller should make sure:
/// - the store has votes since the last committed checkpoint
/// - the votes have at least 1 non-null block
pub fn deduce_new_observation<S: ParentViewStore>(
    store: &S,
    checkpoint: &Checkpoint,
    config: &ObservationConfig,
) -> Result<Observation, Error> {
    let Some(latest_height) = store.max_parent_view_height()? else {
        tracing::info!("no observation yet as height not available");
        return Err(Error::BlockStoreEmpty);
    };

    if latest_height < checkpoint.target_height() {
        tracing::info!("committed vote height more than latest parent view");
        return Err(Error::CommittedParentHeightNotPurged);
    }

    let max_observation_height = checkpoint.target_height() + config.max_observation_range();
    let candidate_height = min(max_observation_height, latest_height);
    tracing::debug!(
        max_observation_height,
        candidate_height,
        "propose observation height"
    );

    // aggregate commitment for the observation
    let mut agg = LinearizedParentBlockView::from(checkpoint);
    for h in checkpoint.target_height() + 1..=candidate_height {
        let Some(p) = store.get(h)? else {
            tracing::debug!(height = h, "not parent block view");
            return Err(Error::MissingBlockView(h, candidate_height));
        };

        agg.append(p)?;
    }

    let observation = agg.into_observation()?;
    tracing::info!(
        height = observation.parent_subnet_height,
        "new observation derived"
    );

    Ok(observation)
}

impl TryFrom<&[u8]> for CertifiedObservation {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(fvm_ipld_encoding::from_slice(bytes)?)
    }
}

impl CertifiedObservation {
    pub fn observation(&self) -> &Observation {
        &self.observation
    }

    pub fn observation_signature(&self) -> &RecoverableECDSASignature {
        &self.observation_signature
    }

    pub fn ensure_valid(&self) -> anyhow::Result<ValidatorKey> {
        let to_sign = fvm_ipld_encoding::to_vec(&self.observation)?;
        let (pk1, _) = self.observation_signature.recover(&to_sign)?;

        let p = Self::envelop_payload(&self.observation_signature, self.certified_at)?;
        let (pk2, _) = self.signature.recover(p.as_slice())?;

        if pk1 != pk2 {
            return Err(anyhow!("public keys not aligned"));
        }

        Ok(ValidatorKey::new(pk1))
    }

    fn envelop_payload(
        observation_sig: &RecoverableECDSASignature,
        certified_at: BlockHeight,
    ) -> anyhow::Result<Bytes> {
        Ok(fvm_ipld_encoding::to_vec(&(observation_sig, certified_at))?)
    }

    pub fn sign(
        ob: Observation,
        certified_at: BlockHeight,
        sk: &SecretKey,
    ) -> anyhow::Result<Self> {
        let obs_payload = fvm_ipld_encoding::to_vec(&ob)?;
        let obs_sig = RecoverableECDSASignature::sign(sk, obs_payload.as_slice())?;

        let p = Self::envelop_payload(&obs_sig, certified_at)?;
        let sig = RecoverableECDSASignature::sign(sk, p.as_slice())?;
        Ok(Self {
            observation: ob,
            observation_signature: obs_sig,
            certified_at,
            signature: sig,
        })
    }
}

impl Observation {
    pub fn new(parent_height: BlockHeight, parent_hash: Bytes, commitment: Bytes) -> Self {
        Self {
            parent_subnet_height: parent_height,
            parent_subnet_hash: parent_hash,
            cumulative_effects_comm: commitment,
        }
    }
}

impl Display for Observation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Observation(parent_height={}, parent_hash={}, commitment={})",
            self.parent_subnet_height,
            hex::encode(&self.parent_subnet_hash),
            hex::encode(&self.cumulative_effects_comm),
        )
    }
}

impl Observation {
    pub fn parent_height(&self) -> BlockHeight {
        self.parent_subnet_height
    }
}

impl ObservationConfig {
    pub fn max_observation_range(&self) -> BlockHeight {
        self.max_observation_range
            .unwrap_or(DEFAULT_MAX_OBSERVATION_RANGE)
    }
}

pub(crate) struct LinearizedParentBlockView {
    parent_height: u64,
    parent_hash: Option<BlockHash>,
    cumulative_effects_comm: Bytes,
}

impl From<&Checkpoint> for LinearizedParentBlockView {
    fn from(value: &Checkpoint) -> Self {
        LinearizedParentBlockView {
            parent_height: value.target_height(),
            parent_hash: Some(value.target_hash().clone()),
            cumulative_effects_comm: value.cumulative_effects_comm().clone(),
        }
    }
}

impl From<&Observation> for LinearizedParentBlockView {
    fn from(value: &Observation) -> Self {
        LinearizedParentBlockView {
            parent_height: value.parent_subnet_height,
            parent_hash: Some(value.parent_subnet_hash.clone()),
            cumulative_effects_comm: value.cumulative_effects_comm.clone(),
        }
    }
}

impl LinearizedParentBlockView {
    fn new_commitment(&mut self, to_append: Bytes) {
        let bytes = [
            self.cumulative_effects_comm.as_slice(),
            to_append.as_slice(),
        ]
        .concat();
        let cid = Cid::new_v1(DAG_CBOR, Code::Blake2b256.digest(&bytes));
        self.cumulative_effects_comm = cid.to_bytes();
    }

    pub fn append(&mut self, view: ParentBlockView) -> Result<(), Error> {
        if self.parent_height + 1 != view.parent_height {
            return Err(Error::NotSequential);
        }

        self.parent_height += 1;

        self.new_commitment(view.effects_commitment()?);

        if let Some(p) = view.payload {
            self.parent_hash = Some(p.parent_hash);
        }

        Ok(())
    }

    pub fn into_observation(self) -> Result<Observation, Error> {
        let Some(hash) = self.parent_hash else {
            return Err(Error::CannotCommitObservationAtNullBlock(
                self.parent_height,
            ));
        };
        Ok(Observation::new(
            self.parent_height,
            hash,
            self.cumulative_effects_comm,
        ))
    }
}
