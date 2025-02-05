pub use subnet_actor_getter_facet::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod subnet_actor_getter_facet {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("activeValidatorsLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "activeValidatorsLimit",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bootstrapped"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bootstrapped"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bottomUpCheckPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "bottomUpCheckPeriod",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bottomUpCheckpointAtEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "bottomUpCheckpointAtEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epoch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exists"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("checkpoint"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                                ::std::boxed::Box::new(
                                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                                ),
                                                                            ),
                                                                        ],
                                                                    ),
                                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                        ],
                                                                    ),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                                ::std::boxed::Box::new(
                                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                                ),
                                                                            ),
                                                                        ],
                                                                    ),
                                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                        ],
                                                                    ),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BottomUpCheckpoint",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bottomUpCheckpointHashAtEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "bottomUpCheckpointHashAtEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epoch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("collateralSource"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("collateralSource"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("supply"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Asset"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("consensus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("consensus"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum ConsensusType"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("crossMsgsHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("crossMsgsHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("messages"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IpcEnvelope[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("genesisBalances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("genesisBalances"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("genesisCircSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("genesisCircSupply"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("genesisValidators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("genesisValidators"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Validator[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getActiveValidators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getActiveValidators",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getActiveValidatorsNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getActiveValidatorsNumber",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBootstrapNodes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBootstrapNodes"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getConfigurationNumbers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getConfigurationNumbers",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getParent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getParent"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct SubnetID"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPower"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPower"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTotalCollateral"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalConfirmedCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalConfirmedCollateral",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalValidatorCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalValidatorCollateral",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalValidatorsNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalValidatorsNumber",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getValidator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ValidatorInfo"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getWaitingValidators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getWaitingValidators",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ipcGatewayAddr"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ipcGatewayAddr"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isActiveValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isActiveValidator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isWaitingValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isWaitingValidator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("killed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("killed"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastBottomUpCheckpointHeight"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastBottomUpCheckpointHeight",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("majorityPercentage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("majorityPercentage"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minActivationCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minActivationCollateral",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minValidators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minValidators"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permissionMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permissionMode"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum PermissionMode"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("powerScale"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("powerScale"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supplySource"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supplySource"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("supply"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Asset"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SUBNETACTORGETTERFACET_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct SubnetActorGetterFacet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SubnetActorGetterFacet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SubnetActorGetterFacet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SubnetActorGetterFacet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SubnetActorGetterFacet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SubnetActorGetterFacet))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SubnetActorGetterFacet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SUBNETACTORGETTERFACET_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `activeValidatorsLimit` (0x3354c3e1) function
        pub fn active_validators_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([51, 84, 195, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bootstrapped` (0x35142c8c) function
        pub fn bootstrapped(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([53, 20, 44, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bottomUpCheckPeriod` (0x06c46853) function
        pub fn bottom_up_check_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([6, 196, 104, 83], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bottomUpCheckpointAtEpoch` (0x4b27aa72) function
        pub fn bottom_up_checkpoint_at_epoch(
            &self,
            epoch: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, BottomUpCheckpoint)> {
            self.0
                .method_hash([75, 39, 170, 114], epoch)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bottomUpCheckpointHashAtEpoch` (0x4b0694e2) function
        pub fn bottom_up_checkpoint_hash_at_epoch(
            &self,
            epoch: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, [u8; 32])> {
            self.0
                .method_hash([75, 6, 148, 226], epoch)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `collateralSource` (0xb6797d3c) function
        pub fn collateral_source(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, Asset> {
            self.0
                .method_hash([182, 121, 125, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `consensus` (0x8ef3f761) function
        pub fn consensus(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([142, 243, 247, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `crossMsgsHash` (0x6b84e383) function
        pub fn cross_msgs_hash(
            &self,
            messages: ::std::vec::Vec<IpcEnvelope>,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([107, 132, 227, 131], messages)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `genesisBalances` (0x903e6930) function
        pub fn genesis_balances(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([144, 62, 105, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `genesisCircSupply` (0x948628a9) function
        pub fn genesis_circ_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([148, 134, 40, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `genesisValidators` (0xd92e8f12) function
        pub fn genesis_validators(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Validator>> {
            self.0
                .method_hash([217, 46, 143, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActiveValidators` (0x9de70258) function
        pub fn get_active_validators(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([157, 231, 2, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActiveValidatorsNumber` (0xc7cda762) function
        pub fn get_active_validators_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([199, 205, 167, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBootstrapNodes` (0x9754b29e) function
        pub fn get_bootstrap_nodes(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([151, 84, 178, 158], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConfigurationNumbers` (0x38a210b3) function
        pub fn get_configuration_numbers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([56, 162, 16, 179], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getParent` (0x80f76021) function
        pub fn get_parent(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, SubnetID> {
            self.0
                .method_hash([128, 247, 96, 33], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPower` (0x5dd9147c) function
        pub fn get_power(
            &self,
            validator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([93, 217, 20, 124], validator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalCollateral` (0xd6eb5910) function
        pub fn get_total_collateral(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([214, 235, 89, 16], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalConfirmedCollateral` (0x332a5ac9) function
        pub fn get_total_confirmed_collateral(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([51, 42, 90, 201], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalValidatorCollateral` (0x1597bf7e) function
        pub fn get_total_validator_collateral(
            &self,
            validator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([21, 151, 191, 126], validator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalValidatorsNumber` (0x52d182d1) function
        pub fn get_total_validators_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([82, 209, 130, 209], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidator` (0x1904bb2e) function
        pub fn get_validator(
            &self,
            validator_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ValidatorInfo> {
            self.0
                .method_hash([25, 4, 187, 46], validator_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWaitingValidators` (0x6ad04c79) function
        pub fn get_waiting_validators(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([106, 208, 76, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ipcGatewayAddr` (0xcfca2824) function
        pub fn ipc_gateway_addr(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([207, 202, 40, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isActiveValidator` (0x40550a1c) function
        pub fn is_active_validator(
            &self,
            validator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([64, 85, 10, 28], validator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isWaitingValidator` (0xd081be03) function
        pub fn is_waiting_validator(
            &self,
            validator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([208, 129, 190, 3], validator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `killed` (0x1f3a0e41) function
        pub fn killed(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([31, 58, 14, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastBottomUpCheckpointHeight` (0x72d0a0e0) function
        pub fn last_bottom_up_checkpoint_height(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([114, 208, 160, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `majorityPercentage` (0x599c7bd1) function
        pub fn majority_percentage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([89, 156, 123, 209], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minActivationCollateral` (0x9e33bd02) function
        pub fn min_activation_collateral(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([158, 51, 189, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minValidators` (0xc5ab2241) function
        pub fn min_validators(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([197, 171, 34, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permissionMode` (0xf0cf6c96) function
        pub fn permission_mode(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([240, 207, 108, 150], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `powerScale` (0xad81e4d6) function
        pub fn power_scale(&self) -> ::ethers::contract::builders::ContractCall<M, i8> {
            self.0
                .method_hash([173, 129, 228, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supplySource` (0x80875df7) function
        pub fn supply_source(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, Asset> {
            self.0
                .method_hash([128, 135, 93, 247], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SubnetActorGetterFacet<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `activeValidatorsLimit` function with signature `activeValidatorsLimit()` and selector `0x3354c3e1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "activeValidatorsLimit", abi = "activeValidatorsLimit()")]
    pub struct ActiveValidatorsLimitCall;
    ///Container type for all input parameters for the `bootstrapped` function with signature `bootstrapped()` and selector `0x35142c8c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "bootstrapped", abi = "bootstrapped()")]
    pub struct BootstrappedCall;
    ///Container type for all input parameters for the `bottomUpCheckPeriod` function with signature `bottomUpCheckPeriod()` and selector `0x06c46853`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "bottomUpCheckPeriod", abi = "bottomUpCheckPeriod()")]
    pub struct BottomUpCheckPeriodCall;
    ///Container type for all input parameters for the `bottomUpCheckpointAtEpoch` function with signature `bottomUpCheckpointAtEpoch(uint256)` and selector `0x4b27aa72`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "bottomUpCheckpointAtEpoch",
        abi = "bottomUpCheckpointAtEpoch(uint256)"
    )]
    pub struct BottomUpCheckpointAtEpochCall {
        pub epoch: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `bottomUpCheckpointHashAtEpoch` function with signature `bottomUpCheckpointHashAtEpoch(uint256)` and selector `0x4b0694e2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "bottomUpCheckpointHashAtEpoch",
        abi = "bottomUpCheckpointHashAtEpoch(uint256)"
    )]
    pub struct BottomUpCheckpointHashAtEpochCall {
        pub epoch: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `collateralSource` function with signature `collateralSource()` and selector `0xb6797d3c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "collateralSource", abi = "collateralSource()")]
    pub struct CollateralSourceCall;
    ///Container type for all input parameters for the `consensus` function with signature `consensus()` and selector `0x8ef3f761`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "consensus", abi = "consensus()")]
    pub struct ConsensusCall;
    ///Container type for all input parameters for the `crossMsgsHash` function with signature `crossMsgsHash((uint8,uint64,uint64,uint256,((uint64,address[]),(uint8,bytes)),((uint64,address[]),(uint8,bytes)),bytes)[])` and selector `0x6b84e383`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "crossMsgsHash",
        abi = "crossMsgsHash((uint8,uint64,uint64,uint256,((uint64,address[]),(uint8,bytes)),((uint64,address[]),(uint8,bytes)),bytes)[])"
    )]
    pub struct CrossMsgsHashCall {
        pub messages: ::std::vec::Vec<IpcEnvelope>,
    }
    ///Container type for all input parameters for the `genesisBalances` function with signature `genesisBalances()` and selector `0x903e6930`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "genesisBalances", abi = "genesisBalances()")]
    pub struct GenesisBalancesCall;
    ///Container type for all input parameters for the `genesisCircSupply` function with signature `genesisCircSupply()` and selector `0x948628a9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "genesisCircSupply", abi = "genesisCircSupply()")]
    pub struct GenesisCircSupplyCall;
    ///Container type for all input parameters for the `genesisValidators` function with signature `genesisValidators()` and selector `0xd92e8f12`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "genesisValidators", abi = "genesisValidators()")]
    pub struct GenesisValidatorsCall;
    ///Container type for all input parameters for the `getActiveValidators` function with signature `getActiveValidators()` and selector `0x9de70258`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getActiveValidators", abi = "getActiveValidators()")]
    pub struct GetActiveValidatorsCall;
    ///Container type for all input parameters for the `getActiveValidatorsNumber` function with signature `getActiveValidatorsNumber()` and selector `0xc7cda762`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getActiveValidatorsNumber", abi = "getActiveValidatorsNumber()")]
    pub struct GetActiveValidatorsNumberCall;
    ///Container type for all input parameters for the `getBootstrapNodes` function with signature `getBootstrapNodes()` and selector `0x9754b29e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getBootstrapNodes", abi = "getBootstrapNodes()")]
    pub struct GetBootstrapNodesCall;
    ///Container type for all input parameters for the `getConfigurationNumbers` function with signature `getConfigurationNumbers()` and selector `0x38a210b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getConfigurationNumbers", abi = "getConfigurationNumbers()")]
    pub struct GetConfigurationNumbersCall;
    ///Container type for all input parameters for the `getParent` function with signature `getParent()` and selector `0x80f76021`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getParent", abi = "getParent()")]
    pub struct GetParentCall;
    ///Container type for all input parameters for the `getPower` function with signature `getPower(address)` and selector `0x5dd9147c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPower", abi = "getPower(address)")]
    pub struct GetPowerCall {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getTotalCollateral` function with signature `getTotalCollateral()` and selector `0xd6eb5910`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getTotalCollateral", abi = "getTotalCollateral()")]
    pub struct GetTotalCollateralCall;
    ///Container type for all input parameters for the `getTotalConfirmedCollateral` function with signature `getTotalConfirmedCollateral()` and selector `0x332a5ac9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getTotalConfirmedCollateral",
        abi = "getTotalConfirmedCollateral()"
    )]
    pub struct GetTotalConfirmedCollateralCall;
    ///Container type for all input parameters for the `getTotalValidatorCollateral` function with signature `getTotalValidatorCollateral(address)` and selector `0x1597bf7e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getTotalValidatorCollateral",
        abi = "getTotalValidatorCollateral(address)"
    )]
    pub struct GetTotalValidatorCollateralCall {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getTotalValidatorsNumber` function with signature `getTotalValidatorsNumber()` and selector `0x52d182d1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getTotalValidatorsNumber", abi = "getTotalValidatorsNumber()")]
    pub struct GetTotalValidatorsNumberCall;
    ///Container type for all input parameters for the `getValidator` function with signature `getValidator(address)` and selector `0x1904bb2e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getValidator", abi = "getValidator(address)")]
    pub struct GetValidatorCall {
        pub validator_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getWaitingValidators` function with signature `getWaitingValidators()` and selector `0x6ad04c79`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getWaitingValidators", abi = "getWaitingValidators()")]
    pub struct GetWaitingValidatorsCall;
    ///Container type for all input parameters for the `ipcGatewayAddr` function with signature `ipcGatewayAddr()` and selector `0xcfca2824`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ipcGatewayAddr", abi = "ipcGatewayAddr()")]
    pub struct IpcGatewayAddrCall;
    ///Container type for all input parameters for the `isActiveValidator` function with signature `isActiveValidator(address)` and selector `0x40550a1c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isActiveValidator", abi = "isActiveValidator(address)")]
    pub struct IsActiveValidatorCall {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isWaitingValidator` function with signature `isWaitingValidator(address)` and selector `0xd081be03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isWaitingValidator", abi = "isWaitingValidator(address)")]
    pub struct IsWaitingValidatorCall {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `killed` function with signature `killed()` and selector `0x1f3a0e41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "killed", abi = "killed()")]
    pub struct KilledCall;
    ///Container type for all input parameters for the `lastBottomUpCheckpointHeight` function with signature `lastBottomUpCheckpointHeight()` and selector `0x72d0a0e0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "lastBottomUpCheckpointHeight",
        abi = "lastBottomUpCheckpointHeight()"
    )]
    pub struct LastBottomUpCheckpointHeightCall;
    ///Container type for all input parameters for the `majorityPercentage` function with signature `majorityPercentage()` and selector `0x599c7bd1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "majorityPercentage", abi = "majorityPercentage()")]
    pub struct MajorityPercentageCall;
    ///Container type for all input parameters for the `minActivationCollateral` function with signature `minActivationCollateral()` and selector `0x9e33bd02`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "minActivationCollateral", abi = "minActivationCollateral()")]
    pub struct MinActivationCollateralCall;
    ///Container type for all input parameters for the `minValidators` function with signature `minValidators()` and selector `0xc5ab2241`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "minValidators", abi = "minValidators()")]
    pub struct MinValidatorsCall;
    ///Container type for all input parameters for the `permissionMode` function with signature `permissionMode()` and selector `0xf0cf6c96`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "permissionMode", abi = "permissionMode()")]
    pub struct PermissionModeCall;
    ///Container type for all input parameters for the `powerScale` function with signature `powerScale()` and selector `0xad81e4d6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "powerScale", abi = "powerScale()")]
    pub struct PowerScaleCall;
    ///Container type for all input parameters for the `supplySource` function with signature `supplySource()` and selector `0x80875df7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "supplySource", abi = "supplySource()")]
    pub struct SupplySourceCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SubnetActorGetterFacetCalls {
        ActiveValidatorsLimit(ActiveValidatorsLimitCall),
        Bootstrapped(BootstrappedCall),
        BottomUpCheckPeriod(BottomUpCheckPeriodCall),
        BottomUpCheckpointAtEpoch(BottomUpCheckpointAtEpochCall),
        BottomUpCheckpointHashAtEpoch(BottomUpCheckpointHashAtEpochCall),
        CollateralSource(CollateralSourceCall),
        Consensus(ConsensusCall),
        CrossMsgsHash(CrossMsgsHashCall),
        GenesisBalances(GenesisBalancesCall),
        GenesisCircSupply(GenesisCircSupplyCall),
        GenesisValidators(GenesisValidatorsCall),
        GetActiveValidators(GetActiveValidatorsCall),
        GetActiveValidatorsNumber(GetActiveValidatorsNumberCall),
        GetBootstrapNodes(GetBootstrapNodesCall),
        GetConfigurationNumbers(GetConfigurationNumbersCall),
        GetParent(GetParentCall),
        GetPower(GetPowerCall),
        GetTotalCollateral(GetTotalCollateralCall),
        GetTotalConfirmedCollateral(GetTotalConfirmedCollateralCall),
        GetTotalValidatorCollateral(GetTotalValidatorCollateralCall),
        GetTotalValidatorsNumber(GetTotalValidatorsNumberCall),
        GetValidator(GetValidatorCall),
        GetWaitingValidators(GetWaitingValidatorsCall),
        IpcGatewayAddr(IpcGatewayAddrCall),
        IsActiveValidator(IsActiveValidatorCall),
        IsWaitingValidator(IsWaitingValidatorCall),
        Killed(KilledCall),
        LastBottomUpCheckpointHeight(LastBottomUpCheckpointHeightCall),
        MajorityPercentage(MajorityPercentageCall),
        MinActivationCollateral(MinActivationCollateralCall),
        MinValidators(MinValidatorsCall),
        PermissionMode(PermissionModeCall),
        PowerScale(PowerScaleCall),
        SupplySource(SupplySourceCall),
    }
    impl ::ethers::core::abi::AbiDecode for SubnetActorGetterFacetCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ActiveValidatorsLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ActiveValidatorsLimit(decoded));
            }
            if let Ok(decoded) = <BootstrappedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Bootstrapped(decoded));
            }
            if let Ok(decoded) = <BottomUpCheckPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BottomUpCheckPeriod(decoded));
            }
            if let Ok(decoded) = <BottomUpCheckpointAtEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BottomUpCheckpointAtEpoch(decoded));
            }
            if let Ok(decoded) = <BottomUpCheckpointHashAtEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BottomUpCheckpointHashAtEpoch(decoded));
            }
            if let Ok(decoded) = <CollateralSourceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CollateralSource(decoded));
            }
            if let Ok(decoded) = <ConsensusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Consensus(decoded));
            }
            if let Ok(decoded) = <CrossMsgsHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CrossMsgsHash(decoded));
            }
            if let Ok(decoded) = <GenesisBalancesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GenesisBalances(decoded));
            }
            if let Ok(decoded) = <GenesisCircSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GenesisCircSupply(decoded));
            }
            if let Ok(decoded) = <GenesisValidatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GenesisValidators(decoded));
            }
            if let Ok(decoded) = <GetActiveValidatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetActiveValidators(decoded));
            }
            if let Ok(decoded) = <GetActiveValidatorsNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetActiveValidatorsNumber(decoded));
            }
            if let Ok(decoded) = <GetBootstrapNodesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBootstrapNodes(decoded));
            }
            if let Ok(decoded) = <GetConfigurationNumbersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetConfigurationNumbers(decoded));
            }
            if let Ok(decoded) = <GetParentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetParent(decoded));
            }
            if let Ok(decoded) = <GetPowerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPower(decoded));
            }
            if let Ok(decoded) = <GetTotalCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalCollateral(decoded));
            }
            if let Ok(decoded) = <GetTotalConfirmedCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalConfirmedCollateral(decoded));
            }
            if let Ok(decoded) = <GetTotalValidatorCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalValidatorCollateral(decoded));
            }
            if let Ok(decoded) = <GetTotalValidatorsNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalValidatorsNumber(decoded));
            }
            if let Ok(decoded) = <GetValidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetValidator(decoded));
            }
            if let Ok(decoded) = <GetWaitingValidatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetWaitingValidators(decoded));
            }
            if let Ok(decoded) = <IpcGatewayAddrCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IpcGatewayAddr(decoded));
            }
            if let Ok(decoded) = <IsActiveValidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsActiveValidator(decoded));
            }
            if let Ok(decoded) = <IsWaitingValidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsWaitingValidator(decoded));
            }
            if let Ok(decoded) = <KilledCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Killed(decoded));
            }
            if let Ok(decoded) = <LastBottomUpCheckpointHeightCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastBottomUpCheckpointHeight(decoded));
            }
            if let Ok(decoded) = <MajorityPercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MajorityPercentage(decoded));
            }
            if let Ok(decoded) = <MinActivationCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinActivationCollateral(decoded));
            }
            if let Ok(decoded) = <MinValidatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinValidators(decoded));
            }
            if let Ok(decoded) = <PermissionModeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PermissionMode(decoded));
            }
            if let Ok(decoded) = <PowerScaleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PowerScale(decoded));
            }
            if let Ok(decoded) = <SupplySourceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupplySource(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SubnetActorGetterFacetCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ActiveValidatorsLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bootstrapped(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BottomUpCheckPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BottomUpCheckpointAtEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BottomUpCheckpointHashAtEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CollateralSource(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Consensus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CrossMsgsHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GenesisBalances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GenesisCircSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GenesisValidators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetActiveValidators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetActiveValidatorsNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBootstrapNodes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetConfigurationNumbers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetParent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalConfirmedCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalValidatorCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalValidatorsNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetWaitingValidators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IpcGatewayAddr(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsActiveValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsWaitingValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Killed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastBottomUpCheckpointHeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MajorityPercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinActivationCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinValidators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PermissionMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PowerScale(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupplySource(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SubnetActorGetterFacetCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActiveValidatorsLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Bootstrapped(element) => ::core::fmt::Display::fmt(element, f),
                Self::BottomUpCheckPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BottomUpCheckpointAtEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BottomUpCheckpointHashAtEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CollateralSource(element) => ::core::fmt::Display::fmt(element, f),
                Self::Consensus(element) => ::core::fmt::Display::fmt(element, f),
                Self::CrossMsgsHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GenesisBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::GenesisCircSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::GenesisValidators(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetActiveValidators(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetActiveValidatorsNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetBootstrapNodes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConfigurationNumbers(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetParent(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPower(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalConfirmedCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalValidatorCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalValidatorsNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetValidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWaitingValidators(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IpcGatewayAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsActiveValidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsWaitingValidator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Killed(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastBottomUpCheckpointHeight(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MajorityPercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinActivationCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinValidators(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermissionMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::PowerScale(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplySource(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ActiveValidatorsLimitCall>
    for SubnetActorGetterFacetCalls {
        fn from(value: ActiveValidatorsLimitCall) -> Self {
            Self::ActiveValidatorsLimit(value)
        }
    }
    impl ::core::convert::From<BootstrappedCall> for SubnetActorGetterFacetCalls {
        fn from(value: BootstrappedCall) -> Self {
            Self::Bootstrapped(value)
        }
    }
    impl ::core::convert::From<BottomUpCheckPeriodCall> for SubnetActorGetterFacetCalls {
        fn from(value: BottomUpCheckPeriodCall) -> Self {
            Self::BottomUpCheckPeriod(value)
        }
    }
    impl ::core::convert::From<BottomUpCheckpointAtEpochCall>
    for SubnetActorGetterFacetCalls {
        fn from(value: BottomUpCheckpointAtEpochCall) -> Self {
            Self::BottomUpCheckpointAtEpoch(value)
        }
    }
    impl ::core::convert::From<BottomUpCheckpointHashAtEpochCall>
    for SubnetActorGetterFacetCalls {
        fn from(value: BottomUpCheckpointHashAtEpochCall) -> Self {
            Self::BottomUpCheckpointHashAtEpoch(value)
        }
    }
    impl ::core::convert::From<CollateralSourceCall> for SubnetActorGetterFacetCalls {
        fn from(value: CollateralSourceCall) -> Self {
            Self::CollateralSource(value)
        }
    }
    impl ::core::convert::From<ConsensusCall> for SubnetActorGetterFacetCalls {
        fn from(value: ConsensusCall) -> Self {
            Self::Consensus(value)
        }
    }
    impl ::core::convert::From<CrossMsgsHashCall> for SubnetActorGetterFacetCalls {
        fn from(value: CrossMsgsHashCall) -> Self {
            Self::CrossMsgsHash(value)
        }
    }
    impl ::core::convert::From<GenesisBalancesCall> for SubnetActorGetterFacetCalls {
        fn from(value: GenesisBalancesCall) -> Self {
            Self::GenesisBalances(value)
        }
    }
    impl ::core::convert::From<GenesisCircSupplyCall> for SubnetActorGetterFacetCalls {
        fn from(value: GenesisCircSupplyCall) -> Self {
            Self::GenesisCircSupply(value)
        }
    }
    impl ::core::convert::From<GenesisValidatorsCall> for SubnetActorGetterFacetCalls {
        fn from(value: GenesisValidatorsCall) -> Self {
            Self::GenesisValidators(value)
        }
    }
    impl ::core::convert::From<GetActiveValidatorsCall> for SubnetActorGetterFacetCalls {
        fn from(value: GetActiveValidatorsCall) -> Self {
            Self::GetActiveValidators(value)
        }
    }
    impl ::core::convert::From<GetActiveValidatorsNumberCall>
    for SubnetActorGetterFacetCalls {
        fn from(value: GetActiveValidatorsNumberCall) -> Self {
            Self::GetActiveValidatorsNumber(value)
        }
    }
    impl ::core::convert::From<GetBootstrapNodesCall> for SubnetActorGetterFacetCalls {
        fn from(value: GetBootstrapNodesCall) -> Self {
            Self::GetBootstrapNodes(value)
        }
    }
    impl ::core::convert::From<GetConfigurationNumbersCall>
    for SubnetActorGetterFacetCalls {
        fn from(value: GetConfigurationNumbersCall) -> Self {
            Self::GetConfigurationNumbers(value)
        }
    }
    impl ::core::convert::From<GetParentCall> for SubnetActorGetterFacetCalls {
        fn from(value: GetParentCall) -> Self {
            Self::GetParent(value)
        }
    }
    impl ::core::convert::From<GetPowerCall> for SubnetActorGetterFacetCalls {
        fn from(value: GetPowerCall) -> Self {
            Self::GetPower(value)
        }
    }
    impl ::core::convert::From<GetTotalCollateralCall> for SubnetActorGetterFacetCalls {
        fn from(value: GetTotalCollateralCall) -> Self {
            Self::GetTotalCollateral(value)
        }
    }
    impl ::core::convert::From<GetTotalConfirmedCollateralCall>
    for SubnetActorGetterFacetCalls {
        fn from(value: GetTotalConfirmedCollateralCall) -> Self {
            Self::GetTotalConfirmedCollateral(value)
        }
    }
    impl ::core::convert::From<GetTotalValidatorCollateralCall>
    for SubnetActorGetterFacetCalls {
        fn from(value: GetTotalValidatorCollateralCall) -> Self {
            Self::GetTotalValidatorCollateral(value)
        }
    }
    impl ::core::convert::From<GetTotalValidatorsNumberCall>
    for SubnetActorGetterFacetCalls {
        fn from(value: GetTotalValidatorsNumberCall) -> Self {
            Self::GetTotalValidatorsNumber(value)
        }
    }
    impl ::core::convert::From<GetValidatorCall> for SubnetActorGetterFacetCalls {
        fn from(value: GetValidatorCall) -> Self {
            Self::GetValidator(value)
        }
    }
    impl ::core::convert::From<GetWaitingValidatorsCall>
    for SubnetActorGetterFacetCalls {
        fn from(value: GetWaitingValidatorsCall) -> Self {
            Self::GetWaitingValidators(value)
        }
    }
    impl ::core::convert::From<IpcGatewayAddrCall> for SubnetActorGetterFacetCalls {
        fn from(value: IpcGatewayAddrCall) -> Self {
            Self::IpcGatewayAddr(value)
        }
    }
    impl ::core::convert::From<IsActiveValidatorCall> for SubnetActorGetterFacetCalls {
        fn from(value: IsActiveValidatorCall) -> Self {
            Self::IsActiveValidator(value)
        }
    }
    impl ::core::convert::From<IsWaitingValidatorCall> for SubnetActorGetterFacetCalls {
        fn from(value: IsWaitingValidatorCall) -> Self {
            Self::IsWaitingValidator(value)
        }
    }
    impl ::core::convert::From<KilledCall> for SubnetActorGetterFacetCalls {
        fn from(value: KilledCall) -> Self {
            Self::Killed(value)
        }
    }
    impl ::core::convert::From<LastBottomUpCheckpointHeightCall>
    for SubnetActorGetterFacetCalls {
        fn from(value: LastBottomUpCheckpointHeightCall) -> Self {
            Self::LastBottomUpCheckpointHeight(value)
        }
    }
    impl ::core::convert::From<MajorityPercentageCall> for SubnetActorGetterFacetCalls {
        fn from(value: MajorityPercentageCall) -> Self {
            Self::MajorityPercentage(value)
        }
    }
    impl ::core::convert::From<MinActivationCollateralCall>
    for SubnetActorGetterFacetCalls {
        fn from(value: MinActivationCollateralCall) -> Self {
            Self::MinActivationCollateral(value)
        }
    }
    impl ::core::convert::From<MinValidatorsCall> for SubnetActorGetterFacetCalls {
        fn from(value: MinValidatorsCall) -> Self {
            Self::MinValidators(value)
        }
    }
    impl ::core::convert::From<PermissionModeCall> for SubnetActorGetterFacetCalls {
        fn from(value: PermissionModeCall) -> Self {
            Self::PermissionMode(value)
        }
    }
    impl ::core::convert::From<PowerScaleCall> for SubnetActorGetterFacetCalls {
        fn from(value: PowerScaleCall) -> Self {
            Self::PowerScale(value)
        }
    }
    impl ::core::convert::From<SupplySourceCall> for SubnetActorGetterFacetCalls {
        fn from(value: SupplySourceCall) -> Self {
            Self::SupplySource(value)
        }
    }
    ///Container type for all return fields from the `activeValidatorsLimit` function with signature `activeValidatorsLimit()` and selector `0x3354c3e1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ActiveValidatorsLimitReturn(pub u16);
    ///Container type for all return fields from the `bootstrapped` function with signature `bootstrapped()` and selector `0x35142c8c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BootstrappedReturn(pub bool);
    ///Container type for all return fields from the `bottomUpCheckPeriod` function with signature `bottomUpCheckPeriod()` and selector `0x06c46853`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BottomUpCheckPeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `bottomUpCheckpointAtEpoch` function with signature `bottomUpCheckpointAtEpoch(uint256)` and selector `0x4b27aa72`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BottomUpCheckpointAtEpochReturn {
        pub exists: bool,
        pub checkpoint: BottomUpCheckpoint,
    }
    ///Container type for all return fields from the `bottomUpCheckpointHashAtEpoch` function with signature `bottomUpCheckpointHashAtEpoch(uint256)` and selector `0x4b0694e2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BottomUpCheckpointHashAtEpochReturn(pub bool, pub [u8; 32]);
    ///Container type for all return fields from the `collateralSource` function with signature `collateralSource()` and selector `0xb6797d3c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CollateralSourceReturn {
        pub supply: Asset,
    }
    ///Container type for all return fields from the `consensus` function with signature `consensus()` and selector `0x8ef3f761`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ConsensusReturn(pub u8);
    ///Container type for all return fields from the `crossMsgsHash` function with signature `crossMsgsHash((uint8,uint64,uint64,uint256,((uint64,address[]),(uint8,bytes)),((uint64,address[]),(uint8,bytes)),bytes)[])` and selector `0x6b84e383`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CrossMsgsHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `genesisBalances` function with signature `genesisBalances()` and selector `0x903e6930`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GenesisBalancesReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `genesisCircSupply` function with signature `genesisCircSupply()` and selector `0x948628a9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GenesisCircSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `genesisValidators` function with signature `genesisValidators()` and selector `0xd92e8f12`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GenesisValidatorsReturn(pub ::std::vec::Vec<Validator>);
    ///Container type for all return fields from the `getActiveValidators` function with signature `getActiveValidators()` and selector `0x9de70258`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetActiveValidatorsReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getActiveValidatorsNumber` function with signature `getActiveValidatorsNumber()` and selector `0xc7cda762`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetActiveValidatorsNumberReturn(pub u16);
    ///Container type for all return fields from the `getBootstrapNodes` function with signature `getBootstrapNodes()` and selector `0x9754b29e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetBootstrapNodesReturn(pub ::std::vec::Vec<::std::string::String>);
    ///Container type for all return fields from the `getConfigurationNumbers` function with signature `getConfigurationNumbers()` and selector `0x38a210b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetConfigurationNumbersReturn(pub u64, pub u64);
    ///Container type for all return fields from the `getParent` function with signature `getParent()` and selector `0x80f76021`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetParentReturn(pub SubnetID);
    ///Container type for all return fields from the `getPower` function with signature `getPower(address)` and selector `0x5dd9147c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetPowerReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalCollateral` function with signature `getTotalCollateral()` and selector `0xd6eb5910`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetTotalCollateralReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalConfirmedCollateral` function with signature `getTotalConfirmedCollateral()` and selector `0x332a5ac9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetTotalConfirmedCollateralReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalValidatorCollateral` function with signature `getTotalValidatorCollateral(address)` and selector `0x1597bf7e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetTotalValidatorCollateralReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalValidatorsNumber` function with signature `getTotalValidatorsNumber()` and selector `0x52d182d1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetTotalValidatorsNumberReturn(pub u16);
    ///Container type for all return fields from the `getValidator` function with signature `getValidator(address)` and selector `0x1904bb2e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetValidatorReturn {
        pub validator: ValidatorInfo,
    }
    ///Container type for all return fields from the `getWaitingValidators` function with signature `getWaitingValidators()` and selector `0x6ad04c79`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetWaitingValidatorsReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `ipcGatewayAddr` function with signature `ipcGatewayAddr()` and selector `0xcfca2824`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IpcGatewayAddrReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isActiveValidator` function with signature `isActiveValidator(address)` and selector `0x40550a1c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsActiveValidatorReturn(pub bool);
    ///Container type for all return fields from the `isWaitingValidator` function with signature `isWaitingValidator(address)` and selector `0xd081be03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsWaitingValidatorReturn(pub bool);
    ///Container type for all return fields from the `killed` function with signature `killed()` and selector `0x1f3a0e41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct KilledReturn(pub bool);
    ///Container type for all return fields from the `lastBottomUpCheckpointHeight` function with signature `lastBottomUpCheckpointHeight()` and selector `0x72d0a0e0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LastBottomUpCheckpointHeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `majorityPercentage` function with signature `majorityPercentage()` and selector `0x599c7bd1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MajorityPercentageReturn(pub u8);
    ///Container type for all return fields from the `minActivationCollateral` function with signature `minActivationCollateral()` and selector `0x9e33bd02`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MinActivationCollateralReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minValidators` function with signature `minValidators()` and selector `0xc5ab2241`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MinValidatorsReturn(pub u64);
    ///Container type for all return fields from the `permissionMode` function with signature `permissionMode()` and selector `0xf0cf6c96`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PermissionModeReturn(pub u8);
    ///Container type for all return fields from the `powerScale` function with signature `powerScale()` and selector `0xad81e4d6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PowerScaleReturn(pub i8);
    ///Container type for all return fields from the `supplySource` function with signature `supplySource()` and selector `0x80875df7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SupplySourceReturn {
        pub supply: Asset,
    }
    ///`Asset(uint8,address)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Asset {
        pub kind: u8,
        pub token_address: ::ethers::core::types::Address,
    }
    ///`BottomUpCheckpoint((uint64,address[]),uint256,bytes32,uint64,(uint8,uint64,uint64,uint256,((uint64,address[]),(uint8,bytes)),((uint64,address[]),(uint8,bytes)),bytes)[],(((uint64,uint64),bytes32)))`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BottomUpCheckpoint {
        pub subnet_id: SubnetID,
        pub block_height: ::ethers::core::types::U256,
        pub block_hash: [u8; 32],
        pub next_configuration_number: u64,
        pub msgs: ::std::vec::Vec<IpcEnvelope>,
        pub activity: CompressedActivityRollup,
    }
    ///`CompressedActivityRollup(((uint64,uint64),bytes32))`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CompressedActivityRollup {
        pub consensus: CompressedSummary,
    }
    ///`AggregatedStats(uint64,uint64)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AggregatedStats {
        pub total_active_validators: u64,
        pub total_num_blocks_committed: u64,
    }
    ///`CompressedSummary((uint64,uint64),bytes32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CompressedSummary {
        pub stats: AggregatedStats,
        pub data_root_commitment: [u8; 32],
    }
    ///`FvmAddress(uint8,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FvmAddress {
        pub addr_type: u8,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///`Ipcaddress((uint64,address[]),(uint8,bytes))`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Ipcaddress {
        pub subnet_id: SubnetID,
        pub raw_address: FvmAddress,
    }
    ///`IpcEnvelope(uint8,uint64,uint64,uint256,((uint64,address[]),(uint8,bytes)),((uint64,address[]),(uint8,bytes)),bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IpcEnvelope {
        pub kind: u8,
        pub local_nonce: u64,
        pub original_nonce: u64,
        pub value: ::ethers::core::types::U256,
        pub to: Ipcaddress,
        pub from: Ipcaddress,
        pub message: ::ethers::core::types::Bytes,
    }
    ///`SubnetID(uint64,address[])`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SubnetID {
        pub root: u64,
        pub route: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///`Validator(uint256,address,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Validator {
        pub weight: ::ethers::core::types::U256,
        pub addr: ::ethers::core::types::Address,
        pub metadata: ::ethers::core::types::Bytes,
    }
    ///`ValidatorInfo(uint256,uint256,uint256,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ValidatorInfo {
        pub federated_power: ::ethers::core::types::U256,
        pub confirmed_collateral: ::ethers::core::types::U256,
        pub total_collateral: ::ethers::core::types::U256,
        pub metadata: ::ethers::core::types::Bytes,
    }
}
