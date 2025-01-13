// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::observation::Observation;
use crate::vote::error::Error;
use crate::vote::payload::{PowerTable, Vote};
use crate::vote::Weight;
use crate::BlockHeight;
use fendermint_crypto::quorum::ECDSACertificate;
use fendermint_vm_genesis::ValidatorKey;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, HashMap};

pub trait VoteStore {
    /// Get the earliest block height of the votes stored
    fn earliest_vote_height(&self) -> Result<Option<BlockHeight>, Error>;

    /// Get the latest block height of the votes stored
    fn latest_vote_height(&self) -> Result<Option<BlockHeight>, Error>;

    /// Store the vote at the target height
    fn store_vote(&mut self, height: BlockHeight, vote: Vote) -> Result<(), Error>;

    /// Checks if the validator has voted at the target block height
    fn has_voted(&self, height: &BlockHeight, validator: &ValidatorKey) -> Result<bool, Error>;

    /// Get the votes at the height.
    fn get_votes_at_height(&self, height: BlockHeight) -> Result<VoteAgg, Error>;

    /// Purge all votes at the specific height. This could be the target height has reached a
    /// quorum and the history is not needed.
    fn purge_votes_at_height(&mut self, height: BlockHeight) -> Result<(), Error>;
}

#[derive(Default)]
pub struct InMemoryVoteStore {
    votes: BTreeMap<BlockHeight, HashMap<ValidatorKey, Vote>>,
}

impl VoteStore for InMemoryVoteStore {
    fn earliest_vote_height(&self) -> Result<Option<BlockHeight>, Error> {
        Ok(self.votes.first_key_value().map(|(k, _)| *k))
    }

    fn latest_vote_height(&self) -> Result<Option<BlockHeight>, Error> {
        Ok(self.votes.last_key_value().map(|(k, _)| *k))
    }

    fn store_vote(&mut self, height: BlockHeight, vote: Vote) -> Result<(), Error> {
        match self.votes.entry(height) {
            Entry::Vacant(_) => {
                let mut map = HashMap::new();
                map.insert(vote.voter(), vote);
                self.votes.insert(height, map);
            }
            Entry::Occupied(mut v) => {
                let key = vote.voter();
                v.get_mut().insert(key, vote);
            }
        }
        Ok(())
    }

    fn has_voted(&self, height: &BlockHeight, validator: &ValidatorKey) -> Result<bool, Error> {
        let Some(votes) = self.votes.get(height) else {
            return Ok(false);
        };
        Ok(votes.contains_key(validator))
    }

    fn get_votes_at_height(&self, height: BlockHeight) -> Result<VoteAgg, Error> {
        let votes = self
            .votes
            .get(&height)
            .map(|v| v.values().collect())
            .unwrap_or_default();
        Ok(VoteAgg::new(votes))
    }

    fn purge_votes_at_height(&mut self, height: BlockHeight) -> Result<(), Error> {
        self.votes.remove(&height);
        Ok(())
    }
}

/// The aggregated votes  from different validators.
pub struct VoteAgg<'a>(HashMap<ValidatorKey, &'a Vote>);

impl<'a> VoteAgg<'a> {
    pub fn new(votes: Vec<&'a Vote>) -> Self {
        let mut map = HashMap::new();
        for v in votes {
            map.insert(v.voter(), v);
        }
        Self(map)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_owned(self) -> Vec<Vote> {
        self.0.into_values().cloned().collect()
    }

    pub fn observation_weights(&self, power_table: &PowerTable) -> Vec<(&Observation, Weight)> {
        let mut votes: Vec<(&Observation, Weight)> = Vec::new();

        for (validator, v) in self.0.iter() {
            let power = power_table.get(validator).cloned().unwrap_or(0);
            if power == 0 {
                continue;
            }

            if let Some(w) = votes.iter_mut().find(|w| w.0 == v.observation()) {
                w.1 += power;
            } else {
                votes.push((v.observation(), power))
            }
        }

        votes
    }

    /// Generate a cert from the ordered validator keys and the target observation as payload
    pub fn generate_cert(
        &self,
        ordered_validators: Vec<(&ValidatorKey, &Weight)>,
        observation: &Observation,
    ) -> Result<ECDSACertificate<Observation>, Error> {
        let mut cert = ECDSACertificate::new_of_size(observation.clone(), ordered_validators.len());

        for (idx, (validator, _)) in ordered_validators.into_iter().enumerate() {
            let Some(vote) = self.0.get(validator) else {
                continue;
            };

            if *vote.observation() == *observation {
                cert.set_signature(
                    idx,
                    validator.public_key(),
                    vote.observation_signature().clone(),
                )
                .map_err(|e| {
                    tracing::error!(err = e.to_string(), "cannot verify signature");
                    Error::VoteCannotBeValidated
                })?;
            }
        }

        Ok(cert)
    }
}

#[cfg(test)]
mod tests {
    use crate::observation::{CertifiedObservation, Observation};
    use crate::vote::payload::Vote;
    use crate::vote::store::VoteAgg;
    use arbitrary::{Arbitrary, Unstructured};
    use fendermint_crypto::SecretKey;
    use fendermint_vm_genesis::ValidatorKey;
    use rand::RngCore;
    use std::collections::HashMap;

    fn random_validator_key() -> (SecretKey, ValidatorKey) {
        let mut rng = rand::thread_rng();
        let sk = SecretKey::random(&mut rng);
        let public_key = sk.public_key();
        (sk, ValidatorKey::new(public_key))
    }

    fn random_observation() -> Observation {
        let mut bytes = [0; 100];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut bytes);

        let mut unstructured = Unstructured::new(&bytes);
        Observation::arbitrary(&mut unstructured).unwrap()
    }

    #[test]
    fn test_works() {
        let validators = (0..3)
            .map(|_| random_validator_key())
            .collect::<Vec<(SecretKey, ValidatorKey)>>();
        let powers = validators
            .iter()
            .map(|v| (v.1.clone(), 1))
            .collect::<Vec<_>>();
        let mut votes = vec![];

        let observation1 = random_observation();
        votes.push(
            Vote::v1_checked(
                CertifiedObservation::sign(observation1.clone(), 100, &validators[0].0).unwrap(),
            )
            .unwrap(),
        );

        let observation2 = random_observation();
        votes.push(
            Vote::v1_checked(
                CertifiedObservation::sign(observation2.clone(), 100, &validators[1].0).unwrap(),
            )
            .unwrap(),
        );
        votes.push(
            Vote::v1_checked(
                CertifiedObservation::sign(observation2.clone(), 100, &validators[2].0).unwrap(),
            )
            .unwrap(),
        );

        let agg = VoteAgg(HashMap::from_iter(votes.iter().map(|v| (v.voter(), v))));
        let mut weights = agg.observation_weights(&HashMap::from_iter(powers));
        weights.sort_by(|a, b| a.1.cmp(&b.1));

        assert_eq!(weights, vec![(&observation1, 1), (&observation2, 2),])
    }
}
