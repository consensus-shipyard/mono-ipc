// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::secp::RecoverableECDSASignature;
use anyhow::anyhow;
use libsecp256k1::PublicKey;
use num_rational::Ratio;
use num_traits::Unsigned;

/// The payload bytes that has been certified by a majority of signer.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "with_serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ECDSACertificate<T> {
    payload: T,
    /// An array of nillable signatures of all active validators in deterministic order.
    signatures: Vec<Option<RecoverableECDSASignature>>,
}

impl<T> ECDSACertificate<T> {
    pub fn new_of_size(payload: T, size: usize) -> Self {
        Self {
            payload,
            signatures: vec![None; size],
        }
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    #[inline]
    fn quorum_threshold<W>(total: W, threshold_ratio: Ratio<W>) -> W
    where
        W: Unsigned + Copy,
    {
        total * *threshold_ratio.numer() / *threshold_ratio.denom() + W::one()
    }
}

#[cfg(feature = "with_serde")]
impl<T: serde::Serialize + PartialEq> ECDSACertificate<T> {
    pub fn set_signature(
        &mut self,
        idx: usize,
        pk: &PublicKey,
        sig: RecoverableECDSASignature,
    ) -> anyhow::Result<()> {
        if !sig.verify(&fvm_ipld_encoding::to_vec(&self.payload)?, pk)? {
            return Err(anyhow!("signature not match publick key"));
        }

        self.signatures[idx] = Some(sig);

        Ok(())
    }

    /// Checks if a quorum is reached from external power table given the payload and sigatures
    pub fn quorum_reached<'a, W, I>(
        &self,
        power_table: I,
        threshold_ratio: Ratio<W>,
    ) -> anyhow::Result<bool>
    where
        W: Copy + Unsigned + PartialOrd,
        I: Iterator<Item = (&'a PublicKey, W)>,
    {
        let (total_weight, signed_weight) = self.calculate_weights::<W, I>(power_table)?;
        Ok(signed_weight >= Self::quorum_threshold::<W>(total_weight, threshold_ratio))
    }

    pub fn calculate_weights<'a, W, I>(&self, power_table: I) -> anyhow::Result<(W, W)>
    where
        W: Copy + Unsigned,
        I: Iterator<Item = (&'a PublicKey, W)>,
    {
        let mut total_weight = W::zero();
        let mut total_pkeys = 0usize;

        let mut signed_weight = W::zero();

        let payload_bytes = fvm_ipld_encoding::to_vec(&self.payload)?;

        for ((pk, weight), maybe_sig) in power_table.zip(self.signatures.iter()) {
            total_weight = total_weight + weight;
            total_pkeys += 1;

            let Some(ref sig) = maybe_sig else {
                continue;
            };

            let (rec_pk, _) = sig.recover(payload_bytes.as_slice())?;
            if *pk != rec_pk {
                return Err(anyhow!("signature not signed by the public key"));
            }

            signed_weight = signed_weight + weight;
        }

        if total_pkeys != self.signatures.len() {
            return Err(anyhow!(
                "invalid number of public keys, expecting: {}, received: {}",
                self.signatures.len(),
                total_pkeys
            ));
        }

        Ok((total_weight, signed_weight))
    }
}

#[cfg(test)]
mod tests {
    use crate::quorum::ECDSACertificate;
    use crate::secp::RecoverableECDSASignature;
    use crate::SecretKey;
    use num_rational::Ratio;
    use rand::{random, thread_rng};

    fn random_secret_keys(num: usize) -> Vec<SecretKey> {
        let mut rng = thread_rng();
        (0..num).map(|_| SecretKey::random(&mut rng)).collect()
    }

    #[test]
    fn test_quorum_all_signed_works() {
        let sks = random_secret_keys(11);

        let payload = vec![10u8; 100];

        let mut quorum = ECDSACertificate::new_of_size(payload.clone(), sks.len());
        let ratio = Ratio::new(2, 3);
        for (i, sk) in sks.iter().enumerate() {
            let sig =
                RecoverableECDSASignature::sign(sk, &fvm_ipld_encoding::to_vec(&payload).unwrap())
                    .unwrap();
            quorum.set_signature(i, &sk.public_key(), sig).unwrap();
        }

        let weights = sks
            .iter()
            .map(|sk| (sk.public_key(), 1u64))
            .collect::<Vec<_>>();
        let is_ok = quorum
            .quorum_reached::<_, _>(weights.iter().map(|(pk, weight)| (pk, *weight)), ratio)
            .unwrap();
        assert!(is_ok);
    }

    #[test]
    fn test_no_quorum_works() {
        let sks = random_secret_keys(11);

        let payload = vec![10u8; 100];
        let ratio = Ratio::new(2, 3);

        let mut quorum = ECDSACertificate::new_of_size(payload.clone(), sks.len());
        for (i, sk) in sks.iter().enumerate() {
            let sig =
                RecoverableECDSASignature::sign(sk, &fvm_ipld_encoding::to_vec(&payload).unwrap())
                    .unwrap();
            if i % 3 == 0 {
                quorum.set_signature(i, &sk.public_key(), sig).unwrap();
            }
        }

        let weights = sks
            .iter()
            .map(|sk| (sk.public_key(), 1u64))
            .collect::<Vec<_>>();
        let is_reached = quorum
            .quorum_reached::<_, _>(weights.iter().map(|(pk, weight)| (pk, *weight)), ratio)
            .unwrap();
        assert!(!is_reached);
    }

    #[test]
    fn test_calculate_weight_all_signed_works() {
        let sks = random_secret_keys(11);

        let payload = vec![10u8; 100];

        let mut quorum = ECDSACertificate::new_of_size(payload.clone(), sks.len());
        for (i, sk) in sks.iter().enumerate() {
            let sig =
                RecoverableECDSASignature::sign(sk, &fvm_ipld_encoding::to_vec(&payload).unwrap())
                    .unwrap();
            quorum.set_signature(i, &sk.public_key(), sig).unwrap();
        }

        let mut total_expected = 0;
        let weights = sks
            .iter()
            .map(|sk| {
                let n = random::<u64>() % 100000;
                total_expected += n;
                (sk.public_key(), n)
            })
            .collect::<Vec<_>>();
        let (total, signed) = quorum
            .calculate_weights::<_, _>(weights.iter().map(|(pk, weight)| (pk, *weight)))
            .unwrap();

        assert_eq!(total, signed);
        assert_eq!(total, total_expected);
    }

    #[test]
    fn test_random_works() {
        let sks = random_secret_keys(11);

        let payload = vec![10u8; 100];

        let mut quorum = ECDSACertificate::new_of_size(payload.clone(), sks.len());
        let mut should_signs = vec![];
        for (i, sk) in sks.iter().enumerate() {
            let sig =
                RecoverableECDSASignature::sign(sk, &fvm_ipld_encoding::to_vec(&payload).unwrap())
                    .unwrap();

            let should_sign = random::<bool>();
            if should_sign {
                quorum.set_signature(i, &sk.public_key(), sig).unwrap();
            }
            should_signs.push(should_sign);
        }

        let mut total_expected = 0;
        let weights = sks
            .iter()
            .map(|sk| {
                let n = random::<u64>() % 100000;
                total_expected += n;
                (sk.public_key(), n)
            })
            .collect::<Vec<_>>();
        let (total, signed) = quorum
            .calculate_weights::<_, _>(weights.iter().map(|(pk, weight)| (pk, *weight)))
            .unwrap();

        let mut signed_expected = 0;
        for (i, should_sign) in should_signs.iter().enumerate() {
            if *should_sign {
                signed_expected += weights[i].1;
            }
        }
        assert_eq!(total, total_expected);
        assert_eq!(signed, signed_expected);
    }
}
