// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

//! Helper methods to convert between Ethereum and FVM data formats.

use ethers_core::types as et;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Eip1559TransactionRequest, TransactionRequest};

pub use fendermint_vm_message::conv::from_eth::*;
use fendermint_vm_message::signed::OriginKind;
use fvm_shared::{error::ExitCode, message::Message};

use crate::error::error_with_revert;
use crate::JsonRpcResult;

fn handle_typed_txn<
    R,
    F1: Fn(&TransactionRequest) -> JsonRpcResult<R>,
    F2: Fn(&Eip1559TransactionRequest) -> JsonRpcResult<R>,
>(
    tx: &TypedTransaction,
    handle_legacy: F1,
    handle_eip1559: F2,
) -> JsonRpcResult<R> {
    match tx {
        TypedTransaction::Legacy(ref t) => handle_legacy(t),
        TypedTransaction::Eip1559(ref t) => handle_eip1559(t),
        _ => error_with_revert(
            ExitCode::USR_ILLEGAL_ARGUMENT,
            "txn type not supported",
            None::<Vec<u8>>,
        ),
    }
}

pub fn derive_origin_kind(tx: &TypedTransaction) -> JsonRpcResult<OriginKind> {
    handle_typed_txn(
        tx,
        |_| Ok(OriginKind::EthereumLegacy),
        |_| Ok(OriginKind::EthereumEIP1559),
    )
}

pub fn to_fvm_message(tx: TypedTransaction) -> JsonRpcResult<Message> {
    handle_typed_txn(
        &tx,
        |r| Ok(fvm_message_from_legacy(r)?),
        |r| Ok(fvm_message_from_eip1559(r)?),
    )
}

/// Turn a request into the DTO returned by the API.
pub fn to_eth_transaction_response(
    tx: &TypedTransaction,
    sig: et::Signature,
) -> JsonRpcResult<et::Transaction> {
    macro_rules! essential_txn_response {
        ($tx: expr, $hash: expr) => {{
            let mut r = et::Transaction::default();

            r.nonce = $tx.nonce.unwrap_or_default();
            r.hash = $hash;
            r.from = $tx.from.unwrap_or_default();
            r.to = $tx.to.clone().and_then(|to| to.as_address().cloned());
            r.value = $tx.value.unwrap_or_default();
            r.gas = $tx.gas.unwrap_or_default();
            r.input = $tx.data.clone().unwrap_or_default();
            r.chain_id = $tx.chain_id.map(|x| et::U256::from(x.as_u64()));
            r.v = et::U64::from(sig.v);
            r.r = sig.r;
            r.s = sig.s;

            r
        }};
    }

    let hash = tx.hash(&sig);

    handle_typed_txn(
        tx,
        |tx| {
            let mut r = essential_txn_response!(tx, hash);
            r.gas_price = tx.gas_price;
            r.transaction_type = Some(0u64.into());
            Ok(r)
        },
        |tx| {
            let mut r = essential_txn_response!(tx, hash);
            r.max_fee_per_gas = tx.max_fee_per_gas;
            r.max_priority_fee_per_gas = tx.max_priority_fee_per_gas;
            // Strictly speaking a "Type 2" transaction should not need to set this, but we do because Blockscout
            // has a database constraint that if a transaction is included in a block this can't be null.
            r.gas_price = Some(
                tx.max_fee_per_gas.unwrap_or_default()
                    + tx.max_priority_fee_per_gas.unwrap_or_default(),
            );
            r.transaction_type = Some(2u64.into());
            r.access_list = Some(tx.access_list.clone());
            Ok(r)
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::conv::from_eth::to_fvm_message;
    use ethers_core::types::transaction::eip2718::TypedTransaction;
    use ethers_core::types::Signature;
    use ethers_core::utils::rlp;
    use fendermint_vm_message::signed::{OriginKind, SignedMessage};
    use fvm_shared::chainid::ChainID;

    #[test]
    fn test_legacy_transaction() {
        let raw_tx = "f8ac821dac850df8475800830186a09465292eeadf1426cd2df1c4793a3d7519f253913b80b844a9059cbb000000000000000000000000cd50511c4e355f2bc3c084d854253cc17b2230bf00000000000000000000000000000000000000000000148a616ad7f95aa0000025a0a4f3a70a01cfb3969c4a12510ebccd7d08250a4d34181123bebae3f865392643a063116147193f2badc611fa20dfa1c339bca299f50e470353ee4f676bc236479d";
        let raw_tx = hex::decode(raw_tx).unwrap();

        let rlp = rlp::Rlp::new(raw_tx.as_ref());
        let (tx, sig): (TypedTransaction, Signature) =
            TypedTransaction::decode_signed(&rlp).unwrap();

        let msg = to_fvm_message(tx).unwrap();

        let signed_msg = SignedMessage {
            origin_kind: OriginKind::EthereumLegacy,
            message: msg,
            signature: fvm_shared::crypto::signature::Signature::new_secp256k1(sig.to_vec()),
        };
        assert!(signed_msg.verify(&ChainID::from(1)).is_ok());
    }
}
