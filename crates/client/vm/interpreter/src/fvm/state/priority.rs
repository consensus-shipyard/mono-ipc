// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::fvm::FvmMessage;
use fvm_shared::econ::TokenAmount;
use num_traits::ToPrimitive;

/// The transaction priority calculator. The priority calculated is used to determine the ordering
/// in the mempool.
pub struct TxnPriorityCalculator {
    base_fee: TokenAmount,
}

impl TxnPriorityCalculator {
    pub fn new(base_fee: TokenAmount) -> Self {
        Self { base_fee }
    }

    pub fn priority(&self, msg: &FvmMessage) -> i64 {
        if msg.gas_fee_cap < self.base_fee {
            return i64::MIN;
        }

        let effective_premium = msg
            .gas_premium
            .clone()
            .min(&msg.gas_fee_cap - &self.base_fee);
        effective_premium.atto().to_i64().unwrap_or(i64::MAX)
    }
}

#[cfg(test)]
mod tests {
    use crate::fvm::state::priority::TxnPriorityCalculator;
    use crate::fvm::FvmMessage;
    use fvm_shared::address::Address;
    use fvm_shared::bigint::BigInt;
    use fvm_shared::econ::TokenAmount;

    fn create_msg(fee_cap: TokenAmount, premium: TokenAmount) -> FvmMessage {
        FvmMessage {
            version: 0,
            from: Address::new_id(10),
            to: Address::new_id(12),
            sequence: 0,
            value: Default::default(),
            method_num: 0,
            params: Default::default(),
            gas_limit: 0,
            gas_fee_cap: fee_cap,
            gas_premium: premium,
        }
    }

    #[test]
    fn priority_calculation() {
        let cal = TxnPriorityCalculator::new(TokenAmount::from_atto(30));

        let msg = create_msg(TokenAmount::from_atto(1), TokenAmount::from_atto(20));
        assert_eq!(cal.priority(&msg), i64::MIN);

        let msg = create_msg(TokenAmount::from_atto(10), TokenAmount::from_atto(20));
        assert_eq!(cal.priority(&msg), i64::MIN);

        let msg = create_msg(TokenAmount::from_atto(35), TokenAmount::from_atto(20));
        assert_eq!(cal.priority(&msg), 5);

        let msg = create_msg(TokenAmount::from_atto(50), TokenAmount::from_atto(20));
        assert_eq!(cal.priority(&msg), 20);

        let msg = create_msg(TokenAmount::from_atto(50), TokenAmount::from_atto(10));
        assert_eq!(cal.priority(&msg), 10);

        let msg = create_msg(
            TokenAmount::from_atto(BigInt::from(i128::MAX)),
            TokenAmount::from_atto(BigInt::from(i128::MAX)),
        );
        assert_eq!(cal.priority(&msg), i64::MAX);
    }
}
