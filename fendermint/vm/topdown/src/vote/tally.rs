// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::observation::Observation;
use crate::vote::error::Error;
use crate::vote::payload::{PowerTable, PowerUpdates, Vote};
use crate::vote::store::VoteStore;
use crate::vote::Weight;
use crate::BlockHeight;
use fendermint_crypto::quorum::ECDSACertificate;
use fendermint_vm_genesis::ValidatorKey;
use num_rational::Ratio;
use std::cmp::Ordering;
use std::collections::HashMap;

/// VoteTally aggregates different votes received from various validators in the network
pub(crate) struct VoteTally<S> {
    /// Current validator weights. These are the ones who will vote on the blocks,
    /// so these are the weights which need to form a quorum.
    power_table: PowerTable,

    /// Index votes received by height and hash, which makes it easy to look up
    /// all the votes for a given block hash and also to verify that a validator
    /// isn't equivocating by trying to vote for two different things at the
    /// same height.
    votes: S,

    /// The latest height that was voted to be finalized and committed to child blockchian
    last_finalized_height: BlockHeight,

    /// The quorum threshold ratio required for a quorum
    quorum_ratio: Ratio<Weight>,
}

impl<S: VoteStore> VoteTally<S> {
    /// Initialize the vote tally from the current power table
    /// and the last finalized block from the ledger.
    pub fn new(
        power_table: Vec<(ValidatorKey, Weight)>,
        last_finalized_height: BlockHeight,
        mut store: S,
    ) -> Result<Self, Error> {
        // purge votes that already committed, no need keep them
        if let Some(h) = store.earliest_vote_height()? {
            for i in h..=last_finalized_height {
                store.purge_votes_at_height(i)?;
            }
        }
        Ok(Self {
            power_table: HashMap::from_iter(power_table),
            votes: store,
            last_finalized_height,
            quorum_ratio: Ratio::new(2, 3),
        })
    }

    /// Check that a validator key is currently part of the power table.
    fn has_power(&self, validator_key: &ValidatorKey) -> bool {
        // For consistency consider validators without power unknown.
        match self.power_table.get(validator_key) {
            None => false,
            Some(weight) => *weight > 0,
        }
    }

    pub fn power_table(&self) -> &HashMap<ValidatorKey, Weight> {
        &self.power_table
    }

    /// Calculate the minimum weight needed for a proposal to pass with the current membership.
    ///
    /// This is inclusive, that is, if the sum of weight is greater or equal to this, it should pass.
    /// The equivalent formula can be found in CometBFT [here](https://github.com/cometbft/cometbft/blob/a8991d63e5aad8be82b90329b55413e3a4933dc0/types/vote_set.go#L307).
    pub fn quorum_threshold(&self) -> Weight {
        let total_weight: Weight = self.power_table.values().sum();
        total_weight * self.quorum_ratio.numer() / self.quorum_ratio.denom() + 1
    }

    /// Return the height of the first entry in the chain.
    ///
    /// This is the block that was finalized *in the ledger*.
    pub fn last_finalized_height(&self) -> BlockHeight {
        self.last_finalized_height
    }

    /// Returns the votes collected in the network at the target height
    pub fn get_votes_at_height(&self, height: BlockHeight) -> Result<Vec<Vote>, Error> {
        let votes = self.votes.get_votes_at_height(height)?;
        Ok(votes.into_owned())
    }

    pub fn check_quorum_cert(&self, cert: &ECDSACertificate<Observation>) -> bool {
        let power_table = self
            .ordered_validators()
            .into_iter()
            .map(|(v, w)| (v.public_key(), *w));
        match cert.quorum_reached(power_table, self.quorum_ratio) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!(err = e.to_string(), "check quorum encountered error");
                false
            }
        }
    }

    /// Dump all the votes that is currently stored in the vote tally.
    /// This is generally a very expensive operation, but good for debugging, use with care
    pub fn dump_votes(&self) -> Result<HashMap<BlockHeight, Vec<Vote>>, Error> {
        let mut r = HashMap::new();

        let Some(latest) = self.votes.latest_vote_height()? else {
            return Ok(r);
        };

        for h in self.last_finalized_height + 1..=latest {
            let votes = self.votes.get_votes_at_height(h)?;
            if votes.is_empty() {
                continue;
            }
            r.insert(h, votes.into_owned());
        }
        Ok(r)
    }

    /// Add a vote we received.
    ///
    /// Returns `true` if this vote was added, `false` if it was ignored as a
    /// duplicate or a height we already finalized, and an error if it's an
    /// equivocation or from a validator we don't know.
    pub fn add_vote(&mut self, vote: Vote) -> Result<bool, Error> {
        let validator = vote.voter();
        let parent_height = vote.observation().parent_height();

        if !self.has_power(&validator) {
            tracing::error!(
                validator = validator.to_string(),
                "validator unknown or has no power"
            );
            return Err(Error::UnpoweredValidator);
        }

        if parent_height < self.last_finalized_height() {
            tracing::debug!(
                parent_height,
                last_finalized_height = self.last_finalized_height(),
                validator = validator.to_string(),
                "reject vote as parent height finalized"
            );
            return Ok(false);
        }

        if self.votes.has_voted(&parent_height, &validator)? {
            tracing::error!(
                parent_height,
                validator = validator.to_string(),
                "equivocation by validator"
            );
            return Err(Error::Equivocation);
        }

        self.votes.store_vote(parent_height, vote)?;

        Ok(true)
    }

    /// Find a block on the (from our perspective) finalized chain that gathered enough votes from validators.
    pub fn find_quorum(&self) -> Result<Option<ECDSACertificate<Observation>>, Error> {
        let quorum_threshold = self.quorum_threshold();
        let Some(max_height) = self.votes.latest_vote_height()? else {
            tracing::info!("vote store has no vote yet, skip finding quorum");
            return Ok(None);
        };

        for h in ((self.last_finalized_height + 1)..=max_height).rev() {
            let votes = self.votes.get_votes_at_height(h)?;

            for (observation, weight) in votes.observation_weights(&self.power_table) {
                tracing::info!(
                    height = h,
                    observation = observation.to_string(),
                    weight,
                    quorum_threshold,
                    "observation and weight"
                );

                if weight >= quorum_threshold {
                    let cert = votes.generate_cert(self.ordered_validators(), observation)?;
                    return Ok(Some(cert));
                }
            }

            tracing::info!(height = h, "no quorum found");
        }

        Ok(None)
    }

    /// Call when a new finalized block is added to the ledger, to clear out all preceding blocks.
    ///
    /// After this operation the minimum item in the chain will the new finalized block.
    pub fn set_finalized(&mut self, block_height: BlockHeight) -> Result<(), Error> {
        let start = if let Some(start) = self.votes.earliest_vote_height()? {
            start
        } else {
            block_height
        };
        for h in start..=block_height {
            self.votes.purge_votes_at_height(h)?;
        }

        self.last_finalized_height = block_height;
        Ok(())
    }

    /// Overwrite the power table after it has changed to a new snapshot.
    ///
    /// This method expects absolute values, it completely replaces the existing powers.
    pub fn set_power_table(&mut self, power_table: PowerUpdates) {
        let power_table = HashMap::from_iter(power_table);
        // We don't actually have to remove the votes of anyone who is no longer a validator,
        // we just have to make sure to handle the case when they are not in the power table.
        self.power_table = power_table;
    }

    /// Update the power table after it has changed with changes.
    ///
    /// This method expects only the updated values, leaving everyone who isn't in it untouched
    pub fn update_power_table(&mut self, power_updates: PowerUpdates) {
        if power_updates.is_empty() {
            return;
        }
        // We don't actually have to remove the votes of anyone who is no longer a validator,
        // we just have to make sure to handle the case when they are not in the power table.
        for (vk, w) in power_updates {
            if w == 0 {
                self.power_table.remove(&vk);
            } else {
                *self.power_table.entry(vk).or_default() = w;
            }
        }
    }

    fn ordered_validators(&self) -> Vec<(&ValidatorKey, &Weight)> {
        let mut sorted_powers = self.power_table.iter().collect::<Vec<_>>();

        sorted_powers.sort_by(|a, b| {
            let cmp = b.1.cmp(a.1);
            if cmp != Ordering::Equal {
                cmp
            } else {
                b.0.cmp(a.0)
            }
        });

        sorted_powers
    }
}

#[cfg(test)]
mod tests {
    use crate::observation::{CertifiedObservation, Observation};
    use crate::vote::error::Error;
    use crate::vote::payload::Vote;
    use crate::vote::store::InMemoryVoteStore;
    use crate::vote::tally::VoteTally;
    use arbitrary::{Arbitrary, Unstructured};
    use fendermint_crypto::SecretKey;
    use fendermint_vm_genesis::ValidatorKey;
    use rand::RngCore;

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
    fn duplicated_vote_not_allowed() {
        let validators = (0..3)
            .map(|_| random_validator_key())
            .collect::<Vec<(SecretKey, ValidatorKey)>>();
        let powers = validators
            .iter()
            .map(|v| (v.1.clone(), 1))
            .collect::<Vec<_>>();
        let mut vote_tally =
            VoteTally::new(powers.clone(), 0, InMemoryVoteStore::default()).unwrap();

        let obs = random_observation();
        let vote = Vote::v1_checked(
            CertifiedObservation::sign(obs.clone(), 100, &validators[0].0).unwrap(),
        )
        .unwrap();
        vote_tally.add_vote(vote).unwrap();

        let mut obs2 = random_observation();
        obs2.parent_subnet_height = obs.parent_subnet_height;
        let vote =
            Vote::v1_checked(CertifiedObservation::sign(obs2, 100, &validators[0].0).unwrap())
                .unwrap();
        assert_eq!(vote_tally.add_vote(vote), Err(Error::Equivocation));
    }

    #[test]
    fn quorum_formed_ok() {
        let validators = (0..3)
            .map(|_| random_validator_key())
            .collect::<Vec<(SecretKey, ValidatorKey)>>();
        let powers = validators
            .iter()
            .map(|v| (v.1.clone(), 1))
            .collect::<Vec<_>>();
        let mut vote_tally =
            VoteTally::new(powers.clone(), 0, InMemoryVoteStore::default()).unwrap();

        let observation = random_observation();

        vote_tally
            .set_finalized(observation.parent_subnet_height - 1)
            .unwrap();

        for validator in validators {
            let certified =
                CertifiedObservation::sign(observation.clone(), 100, &validator.0).unwrap();
            let vote = Vote::v1_checked(certified).unwrap();
            vote_tally.add_vote(vote).unwrap();
        }

        let ob = vote_tally.find_quorum().unwrap().unwrap();
        assert_eq!(*ob.payload(), observation);
    }

    #[test]
    fn no_quorum_formed() {
        let validators_grp1 = (0..2)
            .map(|_| random_validator_key())
            .collect::<Vec<(SecretKey, ValidatorKey)>>();
        let validators_grp2 = (0..2)
            .map(|_| random_validator_key())
            .collect::<Vec<(SecretKey, ValidatorKey)>>();
        let validators = [validators_grp1.as_slice(), validators_grp2.as_slice()]
            .concat()
            .to_vec();

        let powers = validators
            .iter()
            .map(|v| (v.1.clone(), 1))
            .collect::<Vec<_>>();

        let mut vote_tally =
            VoteTally::new(powers.clone(), 0, InMemoryVoteStore::default()).unwrap();

        let observation1 = random_observation();
        let mut observation2 = observation1.clone();
        observation2.parent_subnet_hash = vec![1];

        vote_tally
            .set_finalized(observation1.parent_subnet_height - 1)
            .unwrap();

        for validator in validators_grp1 {
            let certified =
                CertifiedObservation::sign(observation1.clone(), 100, &validator.0).unwrap();
            let vote = Vote::v1_checked(certified).unwrap();
            vote_tally.add_vote(vote).unwrap();
        }
        assert!(vote_tally.find_quorum().unwrap().is_none());

        for validator in validators_grp2 {
            let certified =
                CertifiedObservation::sign(observation2.clone(), 100, &validator.0).unwrap();
            let vote = Vote::v1_checked(certified).unwrap();
            vote_tally.add_vote(vote).unwrap();
        }

        assert!(vote_tally.find_quorum().unwrap().is_none());
    }

    #[test]
    fn new_validators_joined_void_previous_quorum() {
        let validators = (0..3)
            .map(|_| random_validator_key())
            .collect::<Vec<(SecretKey, ValidatorKey)>>();
        let powers = validators
            .iter()
            .map(|v| (v.1.clone(), 1))
            .collect::<Vec<_>>();
        let mut vote_tally =
            VoteTally::new(powers.clone(), 0, InMemoryVoteStore::default()).unwrap();

        let observation = random_observation();

        vote_tally
            .set_finalized(observation.parent_subnet_height - 1)
            .unwrap();

        for validator in validators {
            let certified =
                CertifiedObservation::sign(observation.clone(), 100, &validator.0).unwrap();
            let vote = Vote::v1_checked(certified).unwrap();
            vote_tally.add_vote(vote).unwrap();
        }

        let ob = vote_tally.find_quorum().unwrap().unwrap();
        assert_eq!(*ob.payload(), observation);

        let new_powers = (0..3)
            .map(|_| (random_validator_key().1.clone(), 1))
            .collect::<Vec<_>>();
        vote_tally.update_power_table(new_powers);
        assert_eq!(vote_tally.find_quorum().unwrap(), None);
    }

    #[test]
    fn new_validators_left_formed_quorum() {
        let validators = (0..5)
            .map(|_| random_validator_key())
            .collect::<Vec<(SecretKey, ValidatorKey)>>();
        let powers = validators
            .iter()
            .map(|v| (v.1.clone(), 1))
            .collect::<Vec<_>>();
        let mut vote_tally =
            VoteTally::new(powers.clone(), 0, InMemoryVoteStore::default()).unwrap();

        let observation = random_observation();

        vote_tally
            .set_finalized(observation.parent_subnet_height - 1)
            .unwrap();

        for (count, validator) in validators.iter().enumerate() {
            let certified =
                CertifiedObservation::sign(observation.clone(), 100, &validator.0).unwrap();
            let vote = Vote::v1_checked(certified).unwrap();
            vote_tally.add_vote(vote).unwrap();

            // only 3 validators vote
            if count == 2 {
                break;
            }
        }
        assert!(vote_tally.find_quorum().unwrap().is_none());

        vote_tally.update_power_table(vec![
            (validators[3].1.clone(), 0),
            (validators[4].1.clone(), 0),
        ]);

        let ob = vote_tally.find_quorum().unwrap().unwrap();
        assert_eq!(*ob.payload(), observation);
    }
}
