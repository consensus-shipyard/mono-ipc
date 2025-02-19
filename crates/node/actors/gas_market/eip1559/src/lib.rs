// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use fendermint_actors_api::gas_market::Gas;
use fil_actors_runtime::actor_error;
use fil_actors_runtime::runtime::{ActorCode, Runtime};
use fil_actors_runtime::SYSTEM_ACTOR_ADDR;
use fil_actors_runtime::{actor_dispatch, ActorError};
use fvm_ipld_encoding::tuple::*;
use fvm_shared::econ::TokenAmount;
use fvm_shared::METHOD_CONSTRUCTOR;
use num_derive::FromPrimitive;
use std::cmp::Ordering;

#[cfg(feature = "fil-actor")]
fil_actors_runtime::wasm_trampoline!(Actor);

pub const ACTOR_NAME: &str = "gas_market_eip1559";

pub type SetConstants = Constants;

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone)]
pub struct State {
    pub base_fee: TokenAmount,
    pub constants: Constants,
}

/// Constant params used by EIP-1559.
#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone)]
pub struct Constants {
    pub block_gas_limit: Gas,
    /// The minimal base fee floor when gas utilization is low.
    pub minimal_base_fee: TokenAmount,
    /// Elasticity multiplier as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559).
    pub elasticity_multiplier: u64,
    /// Base fee max change denominator as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559).
    pub base_fee_max_change_denominator: u64,
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone)]
pub struct ConstructorParams {
    initial_base_fee: TokenAmount,
    constants: Constants,
}

pub struct Actor {}

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Constructor = METHOD_CONSTRUCTOR,
    GetConstants = frc42_dispatch::method_hash!("GetConstants"),
    SetConstants = frc42_dispatch::method_hash!("SetConstants"),

    // Standard methods.
    CurrentReading = fendermint_actors_api::gas_market::Method::CurrentReading as u64,
    UpdateUtilization = fendermint_actors_api::gas_market::Method::UpdateUtilization as u64,
}

impl Actor {
    /// Creates the actor
    pub fn constructor(rt: &impl Runtime, params: ConstructorParams) -> Result<(), ActorError> {
        rt.validate_immediate_caller_is(std::iter::once(&SYSTEM_ACTOR_ADDR))?;

        let st = State {
            base_fee: params.initial_base_fee,
            constants: params.constants,
        };

        rt.create(&st)
    }

    fn set_constants(rt: &impl Runtime, constants: SetConstants) -> Result<(), ActorError> {
        rt.validate_immediate_caller_is(std::iter::once(&SYSTEM_ACTOR_ADDR))?;

        rt.transaction(|st: &mut State, _rt| {
            st.constants = constants;
            Ok(())
        })?;

        Ok(())
    }

    fn get_constants(rt: &impl Runtime) -> Result<Constants, ActorError> {
        rt.validate_immediate_caller_accept_any()?;
        rt.state::<State>().map(|s| s.constants)
    }
}

impl fendermint_actors_api::gas_market::GasMarket for Actor {
    fn current_reading(
        rt: &impl Runtime,
    ) -> Result<fendermint_actors_api::gas_market::Reading, ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        let st = rt.state::<State>()?;
        Ok(fendermint_actors_api::gas_market::Reading {
            block_gas_limit: st.constants.block_gas_limit,
            base_fee: st.base_fee,
        })
    }

    fn update_utilization(
        rt: &impl Runtime,
        utilization: fendermint_actors_api::gas_market::Utilization,
    ) -> Result<fendermint_actors_api::gas_market::Reading, ActorError> {
        rt.validate_immediate_caller_is(std::iter::once(&SYSTEM_ACTOR_ADDR))?;

        rt.transaction(|st: &mut State, _rt| {
            st.base_fee = st.next_base_fee(utilization.block_gas_used);
            Ok(fendermint_actors_api::gas_market::Reading {
                block_gas_limit: st.constants.block_gas_limit,
                base_fee: st.base_fee.clone(),
            })
        })
    }
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            // Matching the Filecoin block gas limit. Note that IPC consensus != Filecoin Expected Consensus,
            // TODO
            block_gas_limit: 10_000_000_000,
            // Matching Filecoin's minimal base fee.
            minimal_base_fee: TokenAmount::from_atto(100),
            // Elasticity multiplier as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
            elasticity_multiplier: 2,
            // Base fee max change denominator as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
            base_fee_max_change_denominator: 8,
        }
    }
}

impl State {
    fn next_base_fee(&self, gas_used: Gas) -> TokenAmount {
        let base_fee = self.base_fee.clone();
        let gas_target = self.constants.block_gas_limit / self.constants.elasticity_multiplier;

        match gas_used.cmp(&gas_target) {
            Ordering::Equal => base_fee,
            Ordering::Less => {
                let base_fee_delta = base_fee.atto() * (gas_target - gas_used)
                    / gas_target
                    / self.constants.base_fee_max_change_denominator;
                let base_fee_delta = TokenAmount::from_atto(base_fee_delta);
                if base_fee_delta >= base_fee {
                    self.constants.minimal_base_fee.clone()
                } else {
                    base_fee - base_fee_delta
                }
            }
            Ordering::Greater => {
                let gas_used_delta = gas_used - gas_target;
                let delta = base_fee.atto() * gas_used_delta
                    / gas_target
                    / self.constants.base_fee_max_change_denominator;
                base_fee + TokenAmount::from_atto(delta).max(TokenAmount::from_atto(1))
            }
        }
    }
}

// This import is necessary so that the actor_dispatch macro can find the methods on the GasMarket
// trait, implemented by Self.
use fendermint_actors_api::gas_market::GasMarket;

impl ActorCode for Actor {
    type Methods = Method;

    fn name() -> &'static str {
        ACTOR_NAME
    }

    actor_dispatch! {
        Constructor => constructor,
        SetConstants => set_constants,
        GetConstants => get_constants,

        CurrentReading => current_reading,
        UpdateUtilization => update_utilization,
    }
}

#[cfg(test)]
mod tests {
    use crate::{Actor, Constants, ConstructorParams, Method, State};
    use fendermint_actors_api::gas_market::{Reading, Utilization};
    use fil_actors_runtime::test_utils::{expect_empty, MockRuntime, SYSTEM_ACTOR_CODE_ID};
    use fil_actors_runtime::SYSTEM_ACTOR_ADDR;
    use fvm_ipld_encoding::ipld_block::IpldBlock;
    use fvm_shared::address::Address;
    use fvm_shared::econ::TokenAmount;
    use fvm_shared::error::ExitCode;

    pub fn construct_and_verify() -> MockRuntime {
        let rt = MockRuntime {
            receiver: Address::new_id(10),
            ..Default::default()
        };

        rt.set_caller(*SYSTEM_ACTOR_CODE_ID, SYSTEM_ACTOR_ADDR);
        rt.expect_validate_caller_addr(vec![SYSTEM_ACTOR_ADDR]);

        let result = rt
            .call::<Actor>(
                Method::Constructor as u64,
                IpldBlock::serialize_cbor(&ConstructorParams {
                    initial_base_fee: TokenAmount::from_atto(100),
                    constants: Constants::default(),
                })
                .unwrap(),
            )
            .unwrap();
        expect_empty(result);
        rt.verify();
        rt.reset();

        rt
    }

    #[test]
    fn test_set_ok() {
        let rt = construct_and_verify();

        rt.set_caller(*SYSTEM_ACTOR_CODE_ID, SYSTEM_ACTOR_ADDR);
        rt.expect_validate_caller_addr(vec![SYSTEM_ACTOR_ADDR]);

        let r = rt.call::<Actor>(
            Method::SetConstants as u64,
            IpldBlock::serialize_cbor(&Constants {
                minimal_base_fee: Default::default(),
                elasticity_multiplier: 0,
                base_fee_max_change_denominator: 0,
                block_gas_limit: 20,
            })
            .unwrap(),
        );
        assert!(r.is_ok());

        let s = rt.get_state::<State>();
        assert_eq!(s.constants.block_gas_limit, 20);
    }

    #[test]
    fn test_update_utilization_full_usage() {
        let rt = construct_and_verify();

        rt.set_caller(*SYSTEM_ACTOR_CODE_ID, SYSTEM_ACTOR_ADDR);
        rt.expect_validate_caller_addr(vec![SYSTEM_ACTOR_ADDR]);

        let r = rt.call::<Actor>(
            Method::UpdateUtilization as u64,
            IpldBlock::serialize_cbor(&Utilization {
                // full block usage
                block_gas_used: 10_000_000_000,
            })
            .unwrap(),
        );
        assert!(r.is_ok());

        rt.expect_validate_caller_any();
        let r = rt
            .call::<Actor>(Method::CurrentReading as u64, None)
            .unwrap()
            .unwrap();
        let reading = r.deserialize::<Reading>().unwrap();
        assert_eq!(reading.base_fee, TokenAmount::from_atto(112));
    }

    #[test]
    fn test_update_utilization_equal_usage() {
        let rt = construct_and_verify();

        rt.set_caller(*SYSTEM_ACTOR_CODE_ID, SYSTEM_ACTOR_ADDR);
        rt.expect_validate_caller_addr(vec![SYSTEM_ACTOR_ADDR]);

        let r = rt.call::<Actor>(
            Method::UpdateUtilization as u64,
            IpldBlock::serialize_cbor(&Utilization {
                // full block usage
                block_gas_used: 5_000_000_000,
            })
            .unwrap(),
        );
        assert!(r.is_ok());

        rt.expect_validate_caller_any();
        let r = rt
            .call::<Actor>(Method::CurrentReading as u64, None)
            .unwrap()
            .unwrap();
        let reading = r.deserialize::<Reading>().unwrap();
        assert_eq!(reading.base_fee, TokenAmount::from_atto(100));
    }

    #[test]
    fn test_update_utilization_under_usage() {
        let rt = construct_and_verify();

        rt.set_caller(*SYSTEM_ACTOR_CODE_ID, SYSTEM_ACTOR_ADDR);
        rt.expect_validate_caller_addr(vec![SYSTEM_ACTOR_ADDR]);

        let r = rt.call::<Actor>(
            Method::UpdateUtilization as u64,
            IpldBlock::serialize_cbor(&Utilization {
                // full block usage
                block_gas_used: 100_000_000,
            })
            .unwrap(),
        );
        assert!(r.is_ok());

        rt.expect_validate_caller_any();
        let r = rt
            .call::<Actor>(Method::CurrentReading as u64, None)
            .unwrap()
            .unwrap();
        let reading = r.deserialize::<Reading>().unwrap();
        assert_eq!(reading.base_fee, TokenAmount::from_atto(88));
    }

    #[test]
    fn test_not_allowed() {
        let rt = construct_and_verify();
        rt.set_caller(*SYSTEM_ACTOR_CODE_ID, Address::new_id(1000));
        rt.expect_validate_caller_addr(vec![SYSTEM_ACTOR_ADDR]);

        let code = rt
            .call::<Actor>(
                Method::SetConstants as u64,
                IpldBlock::serialize_cbor(&Constants {
                    minimal_base_fee: TokenAmount::from_atto(10000),
                    elasticity_multiplier: 0,
                    base_fee_max_change_denominator: 0,
                    block_gas_limit: 20,
                })
                .unwrap(),
            )
            .unwrap_err()
            .exit_code();
        assert_eq!(code, ExitCode::USR_FORBIDDEN)
    }
}
