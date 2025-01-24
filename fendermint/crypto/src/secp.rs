// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::SecretKey;
use anyhow::Context;
use libsecp256k1::{recover, sign, PublicKey, RecoveryId};
use multihash::{Code, MultihashDigest};

#[cfg(feature = "with_serde")]
use serde::de::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoverableECDSASignature((u8, [u8; 64]));

impl RecoverableECDSASignature {
    pub fn sign(sk: &SecretKey, payload: &[u8]) -> anyhow::Result<Self> {
        let v = Code::Blake2b256.digest(payload);

        let (sig, rec_id) = sign(&libsecp256k1::Message::parse_slice(v.digest())?, &sk.0);
        Ok(Self((rec_id.serialize(), sig.serialize())))
    }

    pub fn recover(&self, raw_message: &[u8]) -> anyhow::Result<(PublicKey, &[u8; 64])> {
        let v = Code::Blake2b256.digest(raw_message);

        let message = libsecp256k1::Message::parse_slice(v.digest())?;

        let signature = libsecp256k1::Signature::parse_standard(&self.0 .1)
            .context("invalid secp signature")?;
        let rec_id = RecoveryId::parse(self.0 .0)?;

        let pk = recover(&message, &signature, &rec_id)?;
        Ok((pk, &self.0 .1))
    }

    pub fn verify(&self, raw_message: &[u8], pk: &PublicKey) -> anyhow::Result<bool> {
        let (recovered_pk, _) = self.recover(raw_message)?;
        Ok(recovered_pk == *pk)
    }
}

#[cfg(feature = "with_serde")]
impl serde::Serialize for RecoverableECDSASignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let t = (self.0 .0, self.0 .1.to_vec());
        t.serialize(serializer)
    }
}

#[cfg(feature = "with_serde")]
impl<'de> serde::Deserialize<'de> for RecoverableECDSASignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (rec_id, sig) = <(u8, Vec<u8>)>::deserialize(deserializer)?;
        if sig.len() != libsecp256k1::util::SIGNATURE_SIZE {
            return Err(D::Error::custom("invalid secp sig length"));
        }

        let mut v = [0; libsecp256k1::util::SIGNATURE_SIZE];
        v.copy_from_slice(&sig);

        Ok(Self((rec_id, v)))
    }
}

#[cfg(test)]
mod tests {
    use crate::secp::RecoverableECDSASignature;
    use crate::SecretKey;
    use rand::{thread_rng, RngCore};

    #[test]
    fn test_sign_verify() {
        let mut rng = thread_rng();
        let sk = SecretKey::random(&mut rng);

        let mut payload = [0u8; 128];
        rng.fill_bytes(&mut payload);

        let sig = RecoverableECDSASignature::sign(&sk, &payload).unwrap();
        assert!(
            sig.verify(&payload, &sk.public_key()).unwrap(),
            "verify failed"
        );

        let mut payload2 = [0u8; 128];
        rng.fill_bytes(&mut payload2);

        let sig = RecoverableECDSASignature::sign(&sk, &payload).unwrap();
        assert!(
            !sig.verify(&payload2, &sk.public_key()).unwrap(),
            "should verify fail"
        );
    }
}
