pub use subnet_registry_diamond::*;
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
pub mod subnet_registry_diamond {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_diamondCut"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Tuple(
                                    ::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "struct IDiamond.FacetCut[]",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("params"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                            ::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            ],
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "struct SubnetRegistryDiamond.ConstructorParams",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::std::collections::BTreeMap::new(),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DiamondCut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DiamondCut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_diamondCut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_init"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_calldata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotAddFunctionToDiamondThatAlreadyExists",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotAddFunctionToDiamondThatAlreadyExists",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotAddSelectorsToZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotAddSelectorsToZeroAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selectors"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4[]"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotRemoveFunctionThatDoesNotExist",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotRemoveFunctionThatDoesNotExist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotRemoveImmutableFunction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotRemoveImmutableFunction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotReplaceFunctionThatDoesNotExists",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceFunctionThatDoesNotExists",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotReplaceFunctionsFromFacetWithZeroAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceFunctionsFromFacetWithZeroAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selectors"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4[]"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotReplaceImmutableFunction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceImmutableFunction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FacetCannotBeZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FacetCannotBeZero"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FunctionNotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FunctionNotFound"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_functionSelector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GatewayCannotBeZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "GatewayCannotBeZero",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncorrectFacetCutAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "IncorrectFacetCutAction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_action"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IDiamond.FacetCutAction",
                                        ),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InitializationFunctionReverted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InitializationFunctionReverted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_initializationContractAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_calldata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoBytecodeAtAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NoBytecodeAtAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_contractAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NoSelectorsProvidedForFacetForCut",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NoSelectorsProvidedForFacetForCut",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_facetAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "RemoveFacetAddressMustBeZeroAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RemoveFacetAddressMustBeZeroAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_facetAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SUBNETREGISTRYDIAMOND_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4b\0\x1BeWb\0\x1F\xAC\x808\x03\x90\x81b\0\0\x1E\x81b\0\x1CKV[\x91\x829`@\x81\x83\x81\x01\x03\x12b\0\x1BeW\x80Q`\x01`\x01`@\x1B\x03\x81\x11b\0\x1BeW\x81\x01\x91\x80\x82\x01`\x1F\x84\x01\x12\x15b\0\x1BeW\x82Q\x90b\0\0hb\0\0b\x83b\0\x1CqV[b\0\x1CKV[\x93` \x85\x84\x81R\x01` \x81\x94`\x05\x1B\x83\x01\x01\x91\x83\x86\x01\x83\x11b\0\x1BeW` \x81\x01\x91[\x83\x83\x10b\0\x1BiWPPPP` \x83\x01Q\x91`\x01`\x01`@\x1B\x03\x83\x11b\0\x1BeWa\x02\x80\x94\x85\x84\x86\x01\x84\x87\x01\x03\x12b\0\x1BeW`@Q\x95\x86\x01`\x01`\x01`@\x1B\x03\x81\x11\x87\x82\x10\x17b\0\x0B\xB3W`@Rb\0\0\xE7\x84\x86\x01b\0\x1C\x89V[\x86Rb\0\0\xF9` \x85\x87\x01\x01b\0\x1C\x89V[` \x87\x01Rb\0\x01\x0E`@\x85\x87\x01\x01b\0\x1C\x89V[`@\x87\x01Rb\0\x01#``\x85\x87\x01\x01b\0\x1C\x89V[``\x87\x01Rb\0\x018`\x80\x85\x87\x01\x01b\0\x1C\x89V[`\x80\x87\x01Rb\0\x01M`\xA0\x85\x87\x01\x01b\0\x1C\x89V[`\xA0\x87\x01Rb\0\x01b`\xC0\x85\x87\x01\x01b\0\x1C\x89V[`\xC0\x87\x01Rb\0\x01w`\xE0\x85\x87\x01\x01b\0\x1C\x89V[`\xE0\x87\x01Ra\x01\0b\0\x01\x8E\x81\x86\x88\x01\x01b\0\x1C\x89V[\x81\x88\x01Ra\x01 \x90b\0\x01\xA5\x82\x87\x89\x01\x01b\0\x1C\x89V[\x82\x89\x01Ra\x01@\x86\x88\x01\x81\x01Q\x90\x93\x90`\x01`\x01`@\x1B\x03\x81\x11b\0\x1BeWb\0\x01\xD7\x90\x87\x8A\x01\x90\x89\x8B\x01\x01b\0\x1C\x9EV[\x84\x8A\x01Ra\x01`\x87\x89\x01\x81\x01Q\x90\x95\x90`\x01`\x01`@\x1B\x03\x81\x11b\0\x1BeWb\0\x02\t\x90\x88\x8B\x01\x90\x8A\x8C\x01\x01b\0\x1C\x9EV[\x8A\x87\x01R\x88\x88\x01a\x01\x80\x01Q`\x01`\x01`@\x1B\x03\x81\x11b\0\x1BeWb\0\x027\x90\x88\x8B\x01\x90\x8A\x8C\x01\x01b\0\x1C\x9EV[a\x01\x80\x8B\x01Ra\x01\xA0\x88\x8A\x01\x81\x01Q\x90\x97\x90`\x01`\x01`@\x1B\x03\x81\x11b\0\x1BeWb\0\x02k\x90\x82\x8C\x01\x90\x8B\x8D\x01\x01b\0\x1C\x9EV[\x88\x8C\x01Ra\x01\xC0\x89\x8B\x01\x81\x01Q\x90\x99\x90`\x01`\x01`@\x1B\x03\x81\x11b\0\x1BeW\x8Bb\0\x02\x9D\x91\x83\x85\x83\x01\x92\x01\x01b\0\x1C\x9EV[\x8A\x8D\x01Ra\x01\xE0\x8B\x82\x01\x81\x01Q\x90\x9B\x90`\x01`\x01`@\x1B\x03\x81\x11b\0\x1BeW\x8Cb\0\x02\xD1\x8F\x92\x86\x85\x01\x90\x86\x86\x01\x01b\0\x1C\x9EV[\x91\x01R\x80\x82\x01a\x02\0\x01Q`\x01`\x01`@\x1B\x03\x81\x11b\0\x1BeWa\x02\0b\0\x03\x02\x8F\x92\x86\x85\x01\x90\x86\x86\x01\x01b\0\x1C\x9EV[\x91\x01R\x80\x82\x01a\x02 \x01Q`\x01`\x01`@\x1B\x03\x81\x11b\0\x1BeWa\x02 b\0\x033\x8F\x92\x86\x85\x01\x90\x86\x86\x01\x01b\0\x1C\x9EV[\x91\x01R\x80\x82\x01a\x02@\x01Q\x92`\x01`\x01`@\x1B\x03\x84\x11b\0\x1BeWa\x02@b\0\x03i\x8F\x92\x95a\x02`\x96\x85\x01\x90\x86\x86\x01\x01b\0\x1C\x9EV[\x91\x01R\x01\x01Q`\x02\x81\x10\x15b\0\x1BeWa\x02`\x8B\x01R\x89Q`\x01`\x01`\xA0\x1B\x03\x16\x15b\0\x1BSW` \x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15b\0\x1BAW`@\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15b\0\x1BAW``\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15b\0\x1BAW`\x80\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15b\0\x1BAW`\xA0\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15b\0\x1BAW`\xC0\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15b\0\x1BAW`\xE0\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15b\0\x1BAW\x89\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15b\0\x1BAW\x89\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15b\0\x1BAW\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD5\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R` \x82\x01\x92\x90\x92R\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA1`@Q`\x01`\x01`@\x1B\x03` \x82\x01\x90\x81\x11\x90\x82\x11\x17b\0\x0B\xB3W` \x81\x01`@R_\x81R\x82Q_[\x81\x81\x10b\0\x14\xEDWPP`@Q\x92``\x84\x01\x90``\x85RQ\x80\x91R`\x80\x84\x01\x90`\x80\x81`\x05\x1B\x86\x01\x01\x93\x91_\x90[\x82\x82\x10b\0\x14\x93WPPPP\x91b\0\x05S\x81\x92\x7F\x8F\xAAp\x87\x86q\xCC\xD2\x12\xD2\x07q\xB7\x95\xC5\n\xF8\xFD?\xF6\xCF'\xF4\xBD\xE5~]M\xE0\xAE\xB6s\x94_` \x85\x01R\x83\x82\x03`@\x85\x01Rb\0\x1D\x84V[\x03\x90\xA1\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD4` \x90\x81R\x7F}\xFDH\xDD\x9D\xEF\0/\xA9\xB4\xA0[\xD6\xB7&\xA6\xC3\x13\xC3b\xD3\xF3\xE8A=zu \xF0\t\r%\x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x7FM\x7FL\x8A/\xB5\xB3\\\xA3\xC2w\xC98\x88\xB4\x7F\x0F\")\xBD\xCC\xCFfPM\x1B\xA4\x8E\x88\xB8\x81d\x80T\x82\x16\x83\x17\x90UcH\xE2\xB0\x93`\xE0\x1B_\x90\x81R\x7FY\xBAM\xB4\xA2\x13\xE8\x16\x1D\xE5\x97\xB8\xC1\r\xB0\xE7\xE7\xBAZ\xCE\\&\x8E67\x9E$\x9Am-B\xC9\x80T\x90\x92\x16\x83\x17\x90\x91U\x8AQ\x81T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90\x92U\x92\x8B\x01Q\x82T\x82\x16\x90\x84\x16\x17\x90\x91U`@\x8A\x01Q`\x02\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U``\x8A\x01Q`\x03\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U`\x80\x8A\x01Q`\x04\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U`\xA0\x8A\x01Q`\x05\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U`\xC0\x8A\x01Q`\x06\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U`\xE0\x8A\x01Q`\x07\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x91\x89\x01Q`\x08\x80T\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U\x91\x88\x01Q`\t\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U\x85\x01Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11b\0\x0B\xB3Wh\x01\0\0\0\0\0\0\0\0\x82\x11b\0\x0B\xB3W`\nT\x82`\nU\x80\x83\x10b\0\x14\rW[P` \x01\x90`\n_R` _ \x90_[\x81`\x03\x1C\x81\x10b\0\x13\xBEWP`\x07\x19\x81\x16\x80\x82\x03b\0\x13fW[PPPP\x84\x01Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11b\0\x0B\xB3Wh\x01\0\0\0\0\0\0\0\0\x82\x11b\0\x0B\xB3W`\x0BT\x82`\x0BU\x80\x83\x10b\0\x12\xE0W[P` \x01\x90`\x0B_R` _ \x90_[\x81`\x03\x1C\x81\x10b\0\x12\x91WP`\x07\x19\x81\x16\x80\x82\x03b\0\x129W[PPPPa\x01\x80\x84\x01Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11b\0\x0B\xB3Wh\x01\0\0\0\0\0\0\0\0\x82\x11b\0\x0B\xB3W`\x0CT\x82`\x0CU\x80\x83\x10b\0\x11\xB3W[P` \x01\x90`\x0C_R` _ \x90_[\x81`\x03\x1C\x81\x10b\0\x11dWP`\x07\x19\x81\x16\x80\x82\x03b\0\x11\x0CW[PPPP\x83\x01Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11b\0\x0B\xB3Wh\x01\0\0\0\0\0\0\0\0\x82\x11b\0\x0B\xB3W`\rT\x82`\rU\x80\x83\x10b\0\x10\x86W[P` \x01\x90`\r_R` _ \x90_[\x81`\x03\x1C\x81\x10b\0\x107WP`\x07\x19\x81\x16\x80\x82\x03b\0\x0F\xDFW[PPPP\x82\x01Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11b\0\x0B\xB3Wh\x01\0\0\0\0\0\0\0\0\x82\x11b\0\x0B\xB3W`\x0ET\x82`\x0EU\x80\x83\x10b\0\x0F\x80W[P` \x01\x90`\x0E_R` _ \x90_[\x81`\x03\x1C\x81\x10b\0\x0F1WP`\x07\x19\x81\x16\x80\x82\x03b\0\x0E\xD9W[PPPP\x81\x01Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11b\0\x0B\xB3Wh\x01\0\0\0\0\0\0\0\0\x82\x11b\0\x0B\xB3W`\x0FT\x82`\x0FU\x80\x83\x10b\0\x0EzW[P` \x01\x90`\x0F_R` _ \x90_[\x81`\x03\x1C\x81\x10b\0\x0E+WP`\x07\x19\x81\x16\x80\x82\x03b\0\r\xD3W[PPPPa\x02\0\x81\x01Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11b\0\x0B\xB3Wh\x01\0\0\0\0\0\0\0\0\x82\x11b\0\x0B\xB3W`\x10T\x82`\x10U\x80\x83\x10b\0\rtW[P` \x01\x90`\x10_R` _ \x90_[\x81`\x03\x1C\x81\x10b\0\r%WP`\x07\x19\x81\x16\x80\x82\x03b\0\x0C\xCDW[PPPPa\x02 \x81\x01Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11b\0\x0B\xB3Wh\x01\0\0\0\0\0\0\0\0\x82\x11b\0\x0B\xB3W`\x11T\x82`\x11U\x80\x83\x10b\0\x0CnW[P` \x01\x90`\x11_R` _ \x90_[\x81`\x03\x1C\x81\x10b\0\x0C\x1FWP`\x07\x19\x81\x16\x80\x82\x03b\0\x0B\xC7W[PPPPa\x02@\x81\x01Q\x80Q\x91\x90`\x01`\x01`@\x1B\x03\x83\x11b\0\x0B\xB3Wh\x01\0\0\0\0\0\0\0\0\x83\x11b\0\x0B\xB3W`\x12T\x83`\x12U\x80\x84\x10b\0\x0BTW[P` \x01\x91`\x12_R` _ \x81`\x03\x1C\x90_[\x82\x81\x10b\0\x0B\x05WP`\x07\x19\x83\x16\x90\x92\x03\x91\x82b\0\n\xAFW[PPPa\x02`\x91P\x01Q`\x02\x81\x10\x15b\0\n\x9BW`\xFF\x80\x19`\x15T\x16\x91\x16\x17`\x15U`@Qa\x016\x90\x81b\0\x1E6\x829\xF3[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_\x94\x85\x93[\x80\x87\x10b\0\n\xCEWPPa\x02`\x94P\x01U_\x80\x80b\0\niV[\x90\x93` b\0\n\xF9`\x01\x92\x87Q`\xE0\x1C\x90\x8A`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x95\x01\x96\x01\x95\x90b\0\n\xB4V[_\x80[`\x08\x81\x10b\0\x0B\x1FWP\x82\x82\x01U`\x01\x01b\0\nPV[\x96\x90` b\0\x0BJ`\x01\x92\x84Q`\xE0\x1C\x90\x8B`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x92\x01\x97\x01b\0\x0B\x08V[`\x12_R` _ `\x07\x80\x86\x01`\x03\x1C\x82\x01\x92\x01`\x03\x1C\x01\x90`\x1C\x85`\x02\x1B\x16\x80b\0\x0B\x98W[P[\x81\x81\x10b\0\x0B\x8CWPb\0\n<V[_\x81U`\x01\x01b\0\x0B}V[_\x19\x90\x81\x83\x01\x91\x82T\x91` \x03`\x03\x1B\x1C\x16\x90U_b\0\x0B{V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x92_\x93_[\x81\x84\x03\x81\x10b\0\x0B\xE8WPPP`\x03\x1C\x01U_\x80\x80\x80b\0\t\xFEV[\x90\x91\x94` b\0\x0C\x14`\x01\x92\x88Q`\xE0\x1C\x90\x85`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x96\x01\x92\x91\x01b\0\x0B\xCCV[_\x80[`\x08\x81\x10b\0\x0C9WP\x83\x82\x01U`\x01\x01b\0\t\xE4V[\x94\x90` b\0\x0Cd`\x01\x92\x84Q`\xE0\x1C\x90\x89`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x92\x01\x95\x01b\0\x0C\"V[`\x11_R` _ `\x07\x80\x85\x01`\x03\x1C\x82\x01\x92\x01`\x03\x1C\x01\x90`\x1C\x84`\x02\x1B\x16\x80b\0\x0C\xB2W[P[\x81\x81\x10b\0\x0C\xA6WPb\0\t\xD4V[_\x81U`\x01\x01b\0\x0C\x97V[_\x19\x90\x81\x83\x01\x91\x82T\x91` \x03`\x03\x1B\x1C\x16\x90U_b\0\x0C\x95V[\x92_\x93_[\x81\x84\x03\x81\x10b\0\x0C\xEEWPPP`\x03\x1C\x01U_\x80\x80\x80b\0\t\x97V[\x90\x91\x94` b\0\r\x1A`\x01\x92\x88Q`\xE0\x1C\x90\x85`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x96\x01\x92\x91\x01b\0\x0C\xD2V[_\x80[`\x08\x81\x10b\0\r?WP\x83\x82\x01U`\x01\x01b\0\t}V[\x94\x90` b\0\rj`\x01\x92\x84Q`\xE0\x1C\x90\x89`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x92\x01\x95\x01b\0\r(V[`\x10_R` _ `\x07\x80\x85\x01`\x03\x1C\x82\x01\x92\x01`\x03\x1C\x01\x90`\x1C\x84`\x02\x1B\x16\x80b\0\r\xB8W[P[\x81\x81\x10b\0\r\xACWPb\0\tmV[_\x81U`\x01\x01b\0\r\x9DV[_\x19\x90\x81\x83\x01\x91\x82T\x91` \x03`\x03\x1B\x1C\x16\x90U_b\0\r\x9BV[\x92_\x93_[\x81\x84\x03\x81\x10b\0\r\xF4WPPP`\x03\x1C\x01U_\x80\x80\x80b\0\t0V[\x90\x91\x94` b\0\x0E `\x01\x92\x88Q`\xE0\x1C\x90\x85`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x96\x01\x92\x91\x01b\0\r\xD8V[_\x80[`\x08\x81\x10b\0\x0EEWP\x83\x82\x01U`\x01\x01b\0\t\x16V[\x94\x90` b\0\x0Ep`\x01\x92\x84Q`\xE0\x1C\x90\x89`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x92\x01\x95\x01b\0\x0E.V[`\x0F_R` _ `\x07\x80\x85\x01`\x03\x1C\x82\x01\x92\x01`\x03\x1C\x01\x90`\x1C\x84`\x02\x1B\x16\x80b\0\x0E\xBEW[P[\x81\x81\x10b\0\x0E\xB2WPb\0\t\x06V[_\x81U`\x01\x01b\0\x0E\xA3V[_\x19\x90\x81\x83\x01\x91\x82T\x91` \x03`\x03\x1B\x1C\x16\x90U_b\0\x0E\xA1V[\x92_\x93_[\x81\x84\x03\x81\x10b\0\x0E\xFAWPPP`\x03\x1C\x01U_\x80\x80\x80b\0\x08\xCCV[\x90\x91\x94` b\0\x0F&`\x01\x92\x88Q`\xE0\x1C\x90\x85`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x96\x01\x92\x91\x01b\0\x0E\xDEV[_\x80[`\x08\x81\x10b\0\x0FKWP\x83\x82\x01U`\x01\x01b\0\x08\xB2V[\x94\x90` b\0\x0Fv`\x01\x92\x84Q`\xE0\x1C\x90\x89`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x92\x01\x95\x01b\0\x0F4V[`\x0E_R` _ `\x07\x80\x85\x01`\x03\x1C\x82\x01\x92\x01`\x03\x1C\x01\x90`\x1C\x84`\x02\x1B\x16\x80b\0\x0F\xC4W[P[\x81\x81\x10b\0\x0F\xB8WPb\0\x08\xA2V[_\x81U`\x01\x01b\0\x0F\xA9V[_\x19\x90\x81\x83\x01\x91\x82T\x91` \x03`\x03\x1B\x1C\x16\x90U_b\0\x0F\xA7V[\x92_\x93_[\x81\x84\x03\x81\x10b\0\x10\0WPPP`\x03\x1C\x01U_\x80\x80\x80b\0\x08hV[\x90\x91\x94` b\0\x10,`\x01\x92\x88Q`\xE0\x1C\x90\x85`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x96\x01\x92\x91\x01b\0\x0F\xE4V[_\x80[`\x08\x81\x10b\0\x10QWP\x83\x82\x01U`\x01\x01b\0\x08NV[\x94\x90` b\0\x10|`\x01\x92\x84Q`\xE0\x1C\x90\x89`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x92\x01\x95\x01b\0\x10:V[`\r_R\x7F\xD7\xB6\x99\x01\x05q\x91\x01\xDA\xBE\xB7qD\xF2\xA38\\\x803\xAC\xD3\xAF\x97\xE9B:i^\x81\xAD\x1E\xB5`\x02\x84\x90\x1B`\x1C\x16\x80b\0\x10\xE9W[P`\x07\x84\x01`\x03\x1C\x81\x01[`\x07\x83\x01`\x03\x1C\x82\x01\x81\x10b\0\x10\xDDWPPb\0\x08>V[_\x81U`\x01\x01b\0\x10\xC5V[_\x19\x90\x81`\x07\x87\x01`\x03\x1C\x84\x01\x01\x91\x82T\x91` \x03`\x03\x1B\x1C\x16\x90U_b\0\x10\xBAV[\x92_\x93_[\x81\x84\x03\x81\x10b\0\x11-WPPP`\x03\x1C\x01U_\x80\x80\x80b\0\x08\x04V[\x90\x91\x94` b\0\x11Y`\x01\x92\x88Q`\xE0\x1C\x90\x85`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x96\x01\x92\x91\x01b\0\x11\x11V[_\x80[`\x08\x81\x10b\0\x11~WP\x83\x82\x01U`\x01\x01b\0\x07\xEAV[\x94\x90` b\0\x11\xA9`\x01\x92\x84Q`\xE0\x1C\x90\x89`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x92\x01\x95\x01b\0\x11gV[`\x0C_R\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7`\x02\x84\x90\x1B`\x1C\x16\x80b\0\x12\x16W[P`\x07\x84\x01`\x03\x1C\x81\x01[`\x07\x83\x01`\x03\x1C\x82\x01\x81\x10b\0\x12\nWPPb\0\x07\xDAV[_\x81U`\x01\x01b\0\x11\xF2V[_\x19\x90\x81`\x07\x87\x01`\x03\x1C\x84\x01\x01\x91\x82T\x91` \x03`\x03\x1B\x1C\x16\x90U_b\0\x11\xE7V[\x92_\x93_[\x81\x84\x03\x81\x10b\0\x12ZWPPP`\x03\x1C\x01U_\x80\x80\x80b\0\x07\x9DV[\x90\x91\x94` b\0\x12\x86`\x01\x92\x88Q`\xE0\x1C\x90\x85`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x96\x01\x92\x91\x01b\0\x12>V[_\x80[`\x08\x81\x10b\0\x12\xABWP\x83\x82\x01U`\x01\x01b\0\x07\x83V[\x94\x90` b\0\x12\xD6`\x01\x92\x84Q`\xE0\x1C\x90\x89`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x92\x01\x95\x01b\0\x12\x94V[`\x0B_R\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9`\x02\x84\x90\x1B`\x1C\x16\x80b\0\x13CW[P`\x07\x84\x01`\x03\x1C\x81\x01[`\x07\x83\x01`\x03\x1C\x82\x01\x81\x10b\0\x137WPPb\0\x07sV[_\x81U`\x01\x01b\0\x13\x1FV[_\x19\x90\x81`\x07\x87\x01`\x03\x1C\x84\x01\x01\x91\x82T\x91` \x03`\x03\x1B\x1C\x16\x90U_b\0\x13\x14V[\x92_\x93_[\x81\x84\x03\x81\x10b\0\x13\x87WPPP`\x03\x1C\x01U_\x80\x80\x80b\0\x079V[\x90\x91\x94` b\0\x13\xB3`\x01\x92\x88Q`\xE0\x1C\x90\x85`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x96\x01\x92\x91\x01b\0\x13kV[_\x80[`\x08\x81\x10b\0\x13\xD8WP\x83\x82\x01U`\x01\x01b\0\x07\x1FV[\x94\x90` b\0\x14\x03`\x01\x92\x84Q`\xE0\x1C\x90\x89`\x02\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x92\x01\x95\x01b\0\x13\xC1V[`\n_R\x7F\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA8`\x02\x84\x90\x1B`\x1C\x16\x80b\0\x14pW[P`\x07\x84\x01`\x03\x1C\x81\x01[`\x07\x83\x01`\x03\x1C\x82\x01\x81\x10b\0\x14dWPPb\0\x07\x0FV[_\x81U`\x01\x01b\0\x14LV[_\x19\x90\x81`\x07\x87\x01`\x03\x1C\x84\x01\x01\x91\x82T\x91` \x03`\x03\x1B\x1C\x16\x90U_b\0\x14AV[\x90\x91\x92\x94` \x80b\0\x14\xDE`\x01\x93`\x7F\x19\x8B\x82\x03\x01\x86R```@\x8BQ\x87\x80`\xA0\x1B\x03\x81Q\x16\x84Rb\0\x14\xCD\x86\x82\x01Q\x87\x86\x01\x90b\0\x1D7V[\x01Q\x91\x81`@\x82\x01R\x01\x90b\0\x1DEV[\x97\x01\x92\x01\x92\x01\x90\x92\x91b\0\x05\nV[`@b\0\x14\xFB\x82\x87b\0\x1D\x0EV[Q\x01Q`\x01`\x01`\xA0\x1B\x03b\0\x15\x12\x83\x88b\0\x1D\x0EV[QQ\x16\x90\x80Q\x15b\0\x1B(W` b\0\x15,\x84\x89b\0\x1D\x0EV[Q\x01Q`\x03\x81\x10\x15b\0\n\x9BW\x80b\0\x17\\WP\x81\x15b\0\x171Wa\xFF\xFF_\x80Q` b\0\x1Fl\x839\x81Q\x91RT\x16b\0\x15\xA4b\0\x15ib\0\x1C+V[`!\x81R\x7FdiamondCut: Add facet has no cod` \x82\x01R`e`\xF8\x1B`@\x82\x01R\x84b\0\x1D\xF3V[\x81Q\x91_\x91[\x83\x83\x10b\0\x15\xC2WPPPPP`\x01\x90[\x01b\0\x04\xDCV[b\0\x15\xCE\x83\x83b\0\x1D\x0EV[Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16_\x90\x81R_\x80Q` b\0\x1F\x8C\x839\x81Q\x91R` R`@\x90 T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16b\0\x17\x0FWb\0\x16~b\0\x16\x14b\0\x1C\x0BV[\x87\x81Ra\xFF\xFF\x92\x90\x92\x16` \x80\x84\x01\x82\x81R`\x01`\x01`\xE0\x1B\x03\x19\x86\x16_\x90\x81R_\x80Q` b\0\x1F\x8C\x839\x81Q\x91R\x90\x92R`@\x90\x91 \x93Q\x84T\x91Q`\x01`\x01`\xB0\x1B\x03\x19\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17`\xA0\x91\x90\x91\x1Ba\xFF\xFF`\xA0\x1B\x16\x17\x90\x92UV[_\x80Q` b\0\x1Fl\x839\x81Q\x91RT\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15b\0\x0B\xB3Wb\0\x16\xE4b\0\x16\xC5`\x01\x94\x85\x81\x01_\x80Q` b\0\x1Fl\x839\x81Q\x91RUb\0\x1D\xC4V[\x90\x92`\xE0\x1C\x90\x83T\x90`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x90Ua\xFF\xFF\x81\x14b\0\x16\xFBW\x81\x01\x92\x01\x91b\0\x15\xAAV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`@Qc\xEB\xBF]\x07`\xE0\x1B\x81R`\x01`\x01`\xE0\x1B\x03\x19\x83\x16`\x04\x82\x01R`$\x90\xFD[`@Qc\x02\xB8\xDA\x07`\xE2\x1B\x81R` `\x04\x82\x01R\x90\x81\x90b\0\x17X\x90`$\x83\x01\x90b\0\x1DEV[\x03\x90\xFD[`\x01\x81\x03b\0\x18\xCCWP\x81\x15b\0\x18\xA5Wb\0\x17\xBDb\0\x17{b\0\x1C+V[`(\x81R\x7FLibDiamondCut: Replace facet has` \x82\x01Rg no code`\xC0\x1B`@\x82\x01R\x83b\0\x1D\xF3V[\x80Q\x90_[\x82\x81\x10b\0\x17\xD7WPPPP`\x01\x90b\0\x15\xBBV[`\x01`\x01`\xE0\x1B\x03\x19b\0\x17\xEC\x82\x84b\0\x1D\x0EV[Q\x16_\x81\x81R_\x80Q` b\0\x1F\x8C\x839\x81Q\x91R` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x160\x81\x14b\0\x18\x8CW\x85\x81\x14b\0\x18sW\x15b\0\x18[W_\x90\x81R_\x80Q` b\0\x1F\x8C\x839\x81Q\x91R` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x85\x17\x90U`\x01\x01b\0\x17\xC2V[`$\x90`@Q\x90cty\xF99`\xE0\x1B\x82R`\x04\x82\x01R\xFD[`@Qc\x1A\xC6\xCE\x8D`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x90\xFD[`@Qc)\x01\x80m`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x90\xFD[`@Qc\xCD\x98\xA9o`\xE0\x1B\x81R` `\x04\x82\x01R\x90\x81\x90b\0\x17X\x90`$\x83\x01\x90b\0\x1DEV[`\x02\x81\x03b\0\x1B\x06WP_\x80Q` b\0\x1Fl\x839\x81Q\x91RT\x91\x80b\0\x1A\xEEWP\x80Q\x90_[\x82\x81\x10b\0\x19\x08WPPPP`\x01\x90b\0\x15\xBBV[`\x01`\x01`\xE0\x1B\x03\x19b\0\x19\x1D\x82\x84b\0\x1D\x0EV[Q\x16\x90\x81_R_\x80Q` b\0\x1F\x8C\x839\x81Q\x91R` R`@_ \x94b\0\x19Db\0\x1C\x0BV[\x95T`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x88R`\xA0\x91\x90\x91\x1Ca\xFF\xFF\x16` \x88\x01R\x15b\0\x1A\xD5W\x85Q`\x01`\x01`\xA0\x1B\x03\x160\x14b\0\x1A\xBCW\x80\x15b\0\x16\xFBW_\x19\x01\x94\x85a\xFF\xFF` \x83\x01Q\x16\x03b\0\x1A\x15W[P_\x80Q` b\0\x1Fl\x839\x81Q\x91RT\x91\x82\x15b\0\x1A\x01W`\x01\x92_\x19\x01b\0\x19\xC0\x81b\0\x1D\xC4V[c\xFF\xFF\xFF\xFF\x82T\x91`\x03\x1B\x1B\x19\x16\x90U_\x80Q` b\0\x1Fl\x839\x81Q\x91RU_R_\x80Q` b\0\x1F\x8C\x839\x81Q\x91R` R_`@\x81 U\x01b\0\x18\xF3V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[b\0\x1A\xB5\x90a\xFF\xFF` b\0\x1A*\x89b\0\x1D\xC4V[\x90T\x90`\x03\x1B\x1C\x92b\0\x1Ajb\0\x1AF\x84\x84\x84\x01Q\x16b\0\x1D\xC4V[c\xFF\xFF\xFF\xFF\x87\x93\x92\x93\x16\x90\x83T\x90`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x90U\x01Q`\xE0\x92\x90\x92\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16_\x90\x81R_\x80Q` b\0\x1F\x8C\x839\x81Q\x91R` R`@\x90 \x80Ta\xFF\xFF`\xA0\x1B\x19\x16\x91\x90\x92\x16`\xA0\x1Ba\xFF\xFF`\xA0\x1B\x16\x17\x90UV[_b\0\x19\x96V[`@Qc\r\xF5\xFDa`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R`$\x90\xFD[`@Qcz\x08\xA2-`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x90\xFD[`$\x90`@Q\x90c\xD0\x91\xBC\x81`\xE0\x1B\x82R`\x04\x82\x01R\xFD[`@Qc?\xF4\xD2\x0F`\xE1\x1B\x81R`$\x91b\0\x1B&\x90`\x04\x83\x01\x90b\0\x1D7V[\xFD[`@Qc\xE7g\xF9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x90\xFD[`@Qc\x07\xA0CQ`\xE5\x1B\x81R`\x04\x90\xFD[`@Qc\x8B=\xDC3`\xE0\x1B\x81R`\x04\x90\xFD[_\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11b\0\x1BeW\x82\x01``\x91\x82`\x1F\x19\x83\x89\x8C\x01\x03\x01\x12b\0\x1BeW`@Q\x92\x80\x84\x01`\x01`\x01`@\x1B\x03\x81\x11\x85\x82\x10\x17b\0\x0B\xB3W`@Rb\0\x1B\xBA` \x84\x01b\0\x1C\x89V[\x84R`@\x83\x01Q`\x03\x81\x10\x15b\0\x1BeW` \x85\x01R\x82\x01Q\x92`\x01`\x01`@\x1B\x03\x84\x11b\0\x1BeWb\0\x1B\xFA` \x94\x93\x85\x80\x95\x8B\x8E\x01\x92\x01\x01b\0\x1C\x9EV[`@\x82\x01R\x81R\x01\x92\x01\x91b\0\0\x8BV[`@\x80Q\x91\x90\x82\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17b\0\x0B\xB3W`@RV[`@Q\x90``\x82\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17b\0\x0B\xB3W`@RV[`@Q\x91\x90`\x1F\x01`\x1F\x19\x16\x82\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17b\0\x0B\xB3W`@RV[`\x01`\x01`@\x1B\x03\x81\x11b\0\x0B\xB3W`\x05\x1B` \x01\x90V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03b\0\x1BeWV[\x90\x80`\x1F\x83\x01\x12\x15b\0\x1BeW\x81Q\x90` \x91b\0\x1C\xC0b\0\0b\x82b\0\x1CqV[\x93` \x80\x86\x84\x81R\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11b\0\x1BeW` \x01\x90[\x82\x82\x10b\0\x1C\xECWPPPP\x90V[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03b\0\x1BeW\x81R\x90\x83\x01\x90\x83\x01b\0\x1C\xDDV[\x80Q\x82\x10\x15b\0\x1D#W` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x03\x82\x10\x15b\0\n\x9BWRV[\x90\x81Q\x80\x82R` \x80\x80\x93\x01\x93\x01\x91_[\x82\x81\x10b\0\x1DeWPPPP\x90V[\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01b\0\x1DVV[\x91\x90\x82Q\x92\x83\x82R_[\x84\x81\x10b\0\x1D\xAFWPP\x82_` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01b\0\x1D\x8EV[\x90_\x80Q` b\0\x1Fl\x839\x81Q\x91R\x80T\x83\x10\x15b\0\x1D#W_R`\x1C` _ \x83`\x03\x1C\x01\x92`\x02\x1B\x16\x90V[\x80;\x15b\0\x1D\xFFWPPV[`@\x80Qc\x91\x984\xB9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x90\x81\x90b\0\x17X\x90`D\x83\x01\x90b\0\x1D\x84V\xFE`\x80`@R6\x15a\0\x89W_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16\x80\x82R\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD2` R`@\x90\x91 T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\0qW_\x80\x836\x82\x807\x816\x91Z\xF4=_\x80>\x15a\0mW=_\xF3[=_\xFD[`$\x90`@Q\x90c\n\x82\xDDs`\xE3\x1B\x82R`\x04\x82\x01R\xFD[_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16\x80\x82R\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD2` R`@\x90\x91 T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\0\xECW_\x80\x836\x82\x807\x816\x91Z\xF4=_\x80>\x15a\0mW=_\xF3[c\n\x82\xDDs`\xE3\x1B`\x80R`\x84R`$`\x80\xFD\xFE\xA2dipfsX\"\x12 G:nm\xF7\xEF\xE8\xE7\xA2\xDB\x06\xD8\x87t\x03\xC6\\\x07F\x91\xAB\x8Aa\xBB\xA0\x100}2'\xDECdsolcC\0\x08\x17\x003\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD3\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD2";
    /// The bytecode of the contract.
    pub static SUBNETREGISTRYDIAMOND_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R6\x15a\0\x89W_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16\x80\x82R\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD2` R`@\x90\x91 T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\0qW_\x80\x836\x82\x807\x816\x91Z\xF4=_\x80>\x15a\0mW=_\xF3[=_\xFD[`$\x90`@Q\x90c\n\x82\xDDs`\xE3\x1B\x82R`\x04\x82\x01R\xFD[_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16\x80\x82R\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD2` R`@\x90\x91 T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\0\xECW_\x80\x836\x82\x807\x816\x91Z\xF4=_\x80>\x15a\0mW=_\xF3[c\n\x82\xDDs`\xE3\x1B`\x80R`\x84R`$`\x80\xFD\xFE\xA2dipfsX\"\x12 G:nm\xF7\xEF\xE8\xE7\xA2\xDB\x06\xD8\x87t\x03\xC6\\\x07F\x91\xAB\x8Aa\xBB\xA0\x100}2'\xDECdsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static SUBNETREGISTRYDIAMOND_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SubnetRegistryDiamond<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SubnetRegistryDiamond<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SubnetRegistryDiamond<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SubnetRegistryDiamond<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SubnetRegistryDiamond<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SubnetRegistryDiamond))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SubnetRegistryDiamond<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SUBNETREGISTRYDIAMOND_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                SUBNETREGISTRYDIAMOND_ABI.clone(),
                SUBNETREGISTRYDIAMOND_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Gets the contract's `DiamondCut` event
        pub fn diamond_cut_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DiamondCutFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SubnetRegistryDiamondEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SubnetRegistryDiamond<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CannotAddFunctionToDiamondThatAlreadyExists` with signature `CannotAddFunctionToDiamondThatAlreadyExists(bytes4)` and selector `0xebbf5d07`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "CannotAddFunctionToDiamondThatAlreadyExists",
        abi = "CannotAddFunctionToDiamondThatAlreadyExists(bytes4)"
    )]
    pub struct CannotAddFunctionToDiamondThatAlreadyExists {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotAddSelectorsToZeroAddress` with signature `CannotAddSelectorsToZeroAddress(bytes4[])` and selector `0x0ae3681c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "CannotAddSelectorsToZeroAddress",
        abi = "CannotAddSelectorsToZeroAddress(bytes4[])"
    )]
    pub struct CannotAddSelectorsToZeroAddress {
        pub selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///Custom Error type `CannotRemoveFunctionThatDoesNotExist` with signature `CannotRemoveFunctionThatDoesNotExist(bytes4)` and selector `0x7a08a22d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "CannotRemoveFunctionThatDoesNotExist",
        abi = "CannotRemoveFunctionThatDoesNotExist(bytes4)"
    )]
    pub struct CannotRemoveFunctionThatDoesNotExist {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotRemoveImmutableFunction` with signature `CannotRemoveImmutableFunction(bytes4)` and selector `0x6fafeb08`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "CannotRemoveImmutableFunction",
        abi = "CannotRemoveImmutableFunction(bytes4)"
    )]
    pub struct CannotRemoveImmutableFunction {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotReplaceFunctionThatDoesNotExists` with signature `CannotReplaceFunctionThatDoesNotExists(bytes4)` and selector `0x7479f939`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "CannotReplaceFunctionThatDoesNotExists",
        abi = "CannotReplaceFunctionThatDoesNotExists(bytes4)"
    )]
    pub struct CannotReplaceFunctionThatDoesNotExists {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet` with signature `CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(bytes4)` and selector `0x358d9d1a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet",
        abi = "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(bytes4)"
    )]
    pub struct CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotReplaceFunctionsFromFacetWithZeroAddress` with signature `CannotReplaceFunctionsFromFacetWithZeroAddress(bytes4[])` and selector `0xcd98a96f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "CannotReplaceFunctionsFromFacetWithZeroAddress",
        abi = "CannotReplaceFunctionsFromFacetWithZeroAddress(bytes4[])"
    )]
    pub struct CannotReplaceFunctionsFromFacetWithZeroAddress {
        pub selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///Custom Error type `CannotReplaceImmutableFunction` with signature `CannotReplaceImmutableFunction(bytes4)` and selector `0x520300da`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "CannotReplaceImmutableFunction",
        abi = "CannotReplaceImmutableFunction(bytes4)"
    )]
    pub struct CannotReplaceImmutableFunction {
        pub selector: [u8; 4],
    }
    ///Custom Error type `FacetCannotBeZero` with signature `FacetCannotBeZero()` and selector `0xf4086a20`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FacetCannotBeZero", abi = "FacetCannotBeZero()")]
    pub struct FacetCannotBeZero;
    ///Custom Error type `FunctionNotFound` with signature `FunctionNotFound(bytes4)` and selector `0x5416eb98`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FunctionNotFound", abi = "FunctionNotFound(bytes4)")]
    pub struct FunctionNotFound {
        pub function_selector: [u8; 4],
    }
    ///Custom Error type `GatewayCannotBeZero` with signature `GatewayCannotBeZero()` and selector `0x8b3ddc33`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "GatewayCannotBeZero", abi = "GatewayCannotBeZero()")]
    pub struct GatewayCannotBeZero;
    ///Custom Error type `IncorrectFacetCutAction` with signature `IncorrectFacetCutAction(uint8)` and selector `0x7fe9a41e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "IncorrectFacetCutAction", abi = "IncorrectFacetCutAction(uint8)")]
    pub struct IncorrectFacetCutAction {
        pub action: u8,
    }
    ///Custom Error type `InitializationFunctionReverted` with signature `InitializationFunctionReverted(address,bytes)` and selector `0x192105d7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InitializationFunctionReverted",
        abi = "InitializationFunctionReverted(address,bytes)"
    )]
    pub struct InitializationFunctionReverted {
        pub initialization_contract_address: ::ethers::core::types::Address,
        pub calldata: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `NoBytecodeAtAddress` with signature `NoBytecodeAtAddress(address,string)` and selector `0x919834b9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "NoBytecodeAtAddress",
        abi = "NoBytecodeAtAddress(address,string)"
    )]
    pub struct NoBytecodeAtAddress {
        pub contract_address: ::ethers::core::types::Address,
        pub message: ::std::string::String,
    }
    ///Custom Error type `NoSelectorsProvidedForFacetForCut` with signature `NoSelectorsProvidedForFacetForCut(address)` and selector `0xe767f91f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "NoSelectorsProvidedForFacetForCut",
        abi = "NoSelectorsProvidedForFacetForCut(address)"
    )]
    pub struct NoSelectorsProvidedForFacetForCut {
        pub facet_address: ::ethers::core::types::Address,
    }
    ///Custom Error type `RemoveFacetAddressMustBeZeroAddress` with signature `RemoveFacetAddressMustBeZeroAddress(address)` and selector `0xd091bc81`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "RemoveFacetAddressMustBeZeroAddress",
        abi = "RemoveFacetAddressMustBeZeroAddress(address)"
    )]
    pub struct RemoveFacetAddressMustBeZeroAddress {
        pub facet_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SubnetRegistryDiamondErrors {
        CannotAddFunctionToDiamondThatAlreadyExists(
            CannotAddFunctionToDiamondThatAlreadyExists,
        ),
        CannotAddSelectorsToZeroAddress(CannotAddSelectorsToZeroAddress),
        CannotRemoveFunctionThatDoesNotExist(CannotRemoveFunctionThatDoesNotExist),
        CannotRemoveImmutableFunction(CannotRemoveImmutableFunction),
        CannotReplaceFunctionThatDoesNotExists(CannotReplaceFunctionThatDoesNotExists),
        CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
            CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet,
        ),
        CannotReplaceFunctionsFromFacetWithZeroAddress(
            CannotReplaceFunctionsFromFacetWithZeroAddress,
        ),
        CannotReplaceImmutableFunction(CannotReplaceImmutableFunction),
        FacetCannotBeZero(FacetCannotBeZero),
        FunctionNotFound(FunctionNotFound),
        GatewayCannotBeZero(GatewayCannotBeZero),
        IncorrectFacetCutAction(IncorrectFacetCutAction),
        InitializationFunctionReverted(InitializationFunctionReverted),
        NoBytecodeAtAddress(NoBytecodeAtAddress),
        NoSelectorsProvidedForFacetForCut(NoSelectorsProvidedForFacetForCut),
        RemoveFacetAddressMustBeZeroAddress(RemoveFacetAddressMustBeZeroAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SubnetRegistryDiamondErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <CannotAddFunctionToDiamondThatAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotAddFunctionToDiamondThatAlreadyExists(decoded));
            }
            if let Ok(decoded) = <CannotAddSelectorsToZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotAddSelectorsToZeroAddress(decoded));
            }
            if let Ok(decoded) = <CannotRemoveFunctionThatDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotRemoveFunctionThatDoesNotExist(decoded));
            }
            if let Ok(decoded) = <CannotRemoveImmutableFunction as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotRemoveImmutableFunction(decoded));
            }
            if let Ok(decoded) = <CannotReplaceFunctionThatDoesNotExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotReplaceFunctionThatDoesNotExists(decoded));
            }
            if let Ok(decoded) = <CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = <CannotReplaceFunctionsFromFacetWithZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotReplaceFunctionsFromFacetWithZeroAddress(decoded));
            }
            if let Ok(decoded) = <CannotReplaceImmutableFunction as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotReplaceImmutableFunction(decoded));
            }
            if let Ok(decoded) = <FacetCannotBeZero as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FacetCannotBeZero(decoded));
            }
            if let Ok(decoded) = <FunctionNotFound as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FunctionNotFound(decoded));
            }
            if let Ok(decoded) = <GatewayCannotBeZero as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GatewayCannotBeZero(decoded));
            }
            if let Ok(decoded) = <IncorrectFacetCutAction as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncorrectFacetCutAction(decoded));
            }
            if let Ok(decoded) = <InitializationFunctionReverted as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitializationFunctionReverted(decoded));
            }
            if let Ok(decoded) = <NoBytecodeAtAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NoBytecodeAtAddress(decoded));
            }
            if let Ok(decoded) = <NoSelectorsProvidedForFacetForCut as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NoSelectorsProvidedForFacetForCut(decoded));
            }
            if let Ok(decoded) = <RemoveFacetAddressMustBeZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveFacetAddressMustBeZeroAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SubnetRegistryDiamondErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CannotAddFunctionToDiamondThatAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotAddSelectorsToZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotRemoveFunctionThatDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotRemoveImmutableFunction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotReplaceFunctionThatDoesNotExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CannotReplaceFunctionsFromFacetWithZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotReplaceImmutableFunction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FacetCannotBeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FunctionNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GatewayCannotBeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectFacetCutAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializationFunctionReverted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoBytecodeAtAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoSelectorsProvidedForFacetForCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveFacetAddressMustBeZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SubnetRegistryDiamondErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CannotAddFunctionToDiamondThatAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotAddSelectorsToZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotRemoveFunctionThatDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotRemoveImmutableFunction as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceFunctionThatDoesNotExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceFunctionsFromFacetWithZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceImmutableFunction as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FacetCannotBeZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FunctionNotFound as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <GatewayCannotBeZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <IncorrectFacetCutAction as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InitializationFunctionReverted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoBytecodeAtAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoSelectorsProvidedForFacetForCut as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RemoveFacetAddressMustBeZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SubnetRegistryDiamondErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CannotAddFunctionToDiamondThatAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotAddSelectorsToZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotRemoveFunctionThatDoesNotExist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotRemoveImmutableFunction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotReplaceFunctionThatDoesNotExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::CannotReplaceFunctionsFromFacetWithZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotReplaceImmutableFunction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FacetCannotBeZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::FunctionNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::GatewayCannotBeZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncorrectFacetCutAction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializationFunctionReverted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoBytecodeAtAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoSelectorsProvidedForFacetForCut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveFacetAddressMustBeZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SubnetRegistryDiamondErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CannotAddFunctionToDiamondThatAlreadyExists>
    for SubnetRegistryDiamondErrors {
        fn from(value: CannotAddFunctionToDiamondThatAlreadyExists) -> Self {
            Self::CannotAddFunctionToDiamondThatAlreadyExists(value)
        }
    }
    impl ::core::convert::From<CannotAddSelectorsToZeroAddress>
    for SubnetRegistryDiamondErrors {
        fn from(value: CannotAddSelectorsToZeroAddress) -> Self {
            Self::CannotAddSelectorsToZeroAddress(value)
        }
    }
    impl ::core::convert::From<CannotRemoveFunctionThatDoesNotExist>
    for SubnetRegistryDiamondErrors {
        fn from(value: CannotRemoveFunctionThatDoesNotExist) -> Self {
            Self::CannotRemoveFunctionThatDoesNotExist(value)
        }
    }
    impl ::core::convert::From<CannotRemoveImmutableFunction>
    for SubnetRegistryDiamondErrors {
        fn from(value: CannotRemoveImmutableFunction) -> Self {
            Self::CannotRemoveImmutableFunction(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionThatDoesNotExists>
    for SubnetRegistryDiamondErrors {
        fn from(value: CannotReplaceFunctionThatDoesNotExists) -> Self {
            Self::CannotReplaceFunctionThatDoesNotExists(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet>
    for SubnetRegistryDiamondErrors {
        fn from(
            value: CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet,
        ) -> Self {
            Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionsFromFacetWithZeroAddress>
    for SubnetRegistryDiamondErrors {
        fn from(value: CannotReplaceFunctionsFromFacetWithZeroAddress) -> Self {
            Self::CannotReplaceFunctionsFromFacetWithZeroAddress(value)
        }
    }
    impl ::core::convert::From<CannotReplaceImmutableFunction>
    for SubnetRegistryDiamondErrors {
        fn from(value: CannotReplaceImmutableFunction) -> Self {
            Self::CannotReplaceImmutableFunction(value)
        }
    }
    impl ::core::convert::From<FacetCannotBeZero> for SubnetRegistryDiamondErrors {
        fn from(value: FacetCannotBeZero) -> Self {
            Self::FacetCannotBeZero(value)
        }
    }
    impl ::core::convert::From<FunctionNotFound> for SubnetRegistryDiamondErrors {
        fn from(value: FunctionNotFound) -> Self {
            Self::FunctionNotFound(value)
        }
    }
    impl ::core::convert::From<GatewayCannotBeZero> for SubnetRegistryDiamondErrors {
        fn from(value: GatewayCannotBeZero) -> Self {
            Self::GatewayCannotBeZero(value)
        }
    }
    impl ::core::convert::From<IncorrectFacetCutAction> for SubnetRegistryDiamondErrors {
        fn from(value: IncorrectFacetCutAction) -> Self {
            Self::IncorrectFacetCutAction(value)
        }
    }
    impl ::core::convert::From<InitializationFunctionReverted>
    for SubnetRegistryDiamondErrors {
        fn from(value: InitializationFunctionReverted) -> Self {
            Self::InitializationFunctionReverted(value)
        }
    }
    impl ::core::convert::From<NoBytecodeAtAddress> for SubnetRegistryDiamondErrors {
        fn from(value: NoBytecodeAtAddress) -> Self {
            Self::NoBytecodeAtAddress(value)
        }
    }
    impl ::core::convert::From<NoSelectorsProvidedForFacetForCut>
    for SubnetRegistryDiamondErrors {
        fn from(value: NoSelectorsProvidedForFacetForCut) -> Self {
            Self::NoSelectorsProvidedForFacetForCut(value)
        }
    }
    impl ::core::convert::From<RemoveFacetAddressMustBeZeroAddress>
    for SubnetRegistryDiamondErrors {
        fn from(value: RemoveFacetAddressMustBeZeroAddress) -> Self {
            Self::RemoveFacetAddressMustBeZeroAddress(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DiamondCut",
        abi = "DiamondCut((address,uint8,bytes4[])[],address,bytes)"
    )]
    pub struct DiamondCutFilter {
        pub diamond_cut: ::std::vec::Vec<FacetCut>,
        pub init: ::ethers::core::types::Address,
        pub calldata: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        pub old_owner: ::ethers::core::types::Address,
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SubnetRegistryDiamondEvents {
        DiamondCutFilter(DiamondCutFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for SubnetRegistryDiamondEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DiamondCutFilter::decode_log(log) {
                return Ok(SubnetRegistryDiamondEvents::DiamondCutFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    SubnetRegistryDiamondEvents::OwnershipTransferredFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SubnetRegistryDiamondEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DiamondCutFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DiamondCutFilter> for SubnetRegistryDiamondEvents {
        fn from(value: DiamondCutFilter) -> Self {
            Self::DiamondCutFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for SubnetRegistryDiamondEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///`FacetCut(address,uint8,bytes4[])`
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
    pub struct FacetCut {
        pub facet_address: ::ethers::core::types::Address,
        pub action: u8,
        pub function_selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///`ConstructorParams(address,address,address,address,address,address,address,address,address,address,bytes4[],bytes4[],bytes4[],bytes4[],bytes4[],bytes4[],bytes4[],bytes4[],bytes4[],uint8)`
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
    pub struct ConstructorParams {
        pub gateway: ::ethers::core::types::Address,
        pub getter_facet: ::ethers::core::types::Address,
        pub manager_facet: ::ethers::core::types::Address,
        pub rewarder_facet: ::ethers::core::types::Address,
        pub checkpointer_facet: ::ethers::core::types::Address,
        pub pauser_facet: ::ethers::core::types::Address,
        pub diamond_cut_facet: ::ethers::core::types::Address,
        pub diamond_loupe_facet: ::ethers::core::types::Address,
        pub ownership_facet: ::ethers::core::types::Address,
        pub activity_facet: ::ethers::core::types::Address,
        pub subnet_actor_getter_selectors: ::std::vec::Vec<[u8; 4]>,
        pub subnet_actor_manager_selectors: ::std::vec::Vec<[u8; 4]>,
        pub subnet_actor_rewarder_selectors: ::std::vec::Vec<[u8; 4]>,
        pub subnet_actor_checkpointer_selectors: ::std::vec::Vec<[u8; 4]>,
        pub subnet_actor_pauser_selectors: ::std::vec::Vec<[u8; 4]>,
        pub subnet_actor_diamond_cut_selectors: ::std::vec::Vec<[u8; 4]>,
        pub subnet_actor_diamond_loupe_selectors: ::std::vec::Vec<[u8; 4]>,
        pub subnet_actor_ownership_selectors: ::std::vec::Vec<[u8; 4]>,
        pub subnet_actor_activity_selectors: ::std::vec::Vec<[u8; 4]>,
        pub creation_privileges: u8,
    }
}
