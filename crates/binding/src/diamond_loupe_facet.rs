pub use diamond_loupe_facet::*;
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
pub mod diamond_loupe_facet {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("facetAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facetAddress"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("facetAddress_"),
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
                    ::std::borrow::ToOwned::to_owned("facetAddresses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facetAddresses"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("facetAddresses_"),
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
                    ::std::borrow::ToOwned::to_owned("facetFunctionSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "facetFunctionSelectors",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_facet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_facetFunctionSelectors",
                                    ),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("facets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facets"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("facets_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
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
                                            "struct IDiamondLoupe.Facet[]",
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DIAMONDLOUPEFACET_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x08A\x90\x81a\0\x1B\x829\xF3[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\0dW\x80cR\xEFk,\x14a\0_W\x80cz\x0E\xD6'\x14a\0ZW\x80c\xAD\xFC\xA1^\x14a\0UWc\xCD\xFF\xAC\xC6\x14a\0PW_\x80\xFD[a\x05\xEDV[a\x055V[a\x02\xE9V[a\x01 V[4a\0\xB5W`\x01`\x01`\xE0\x1B\x03\x19a\0{6a\0\xB9V[\x16_R\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD4` R`\xFF`@_ T\x16\x15\x15`\x80R` `\x80\xF3[_\x80\xFD[` \x90`\x03\x19\x01\x12a\0\xB5W`\x045`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\0\xB5W\x90V[` \x90` `@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91_[\x82\x81\x10a\x01\x03WPPPP\x90V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\0\xF5V[4a\0\xB5W_6`\x03\x19\x01\x12a\0\xB5W_\x80Q` a\x07\xEC\x839\x81Q\x91RTa\x01H\x81a\x06\x9FV[_\x80\x92[\x80\x84\x10a\x01hW\x81\x83R`@Q\x80a\x01d\x85\x82a\0\xDCV[\x03\x90\xF3[\x90a\x01\x9Aa\x01\x8Da\x01\x88a\x01{\x87a\x06\xE0V[\x90T\x90`\x03\x1B\x1C`\xE0\x1B\x90V[a\x06\x12V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[_`\x01`\x01`\xA0\x1B\x03\x82\x16\x81[\x84\x81\x10a\x01\xF1W[PPa\x01\xE7W\x81a\x01\xD8a\x01\xDD\x92a\x01\xC9`\x01\x95\x88a\x07<V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x07dV[\x93[\x01\x92\x90a\x01LV[P\x92`\x01\x90a\x01\xDFV[a\x02\x1Aa\x02\x0Ea\x02\x01\x83\x8Aa\x07<V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x82\x14a\x02(W`\x01\x01a\x01\xA7V[PPP`\x01_\x80a\x01\xAFV[` \x80\x82\x01\x90\x80\x83R\x83Q\x80\x92R`@\x92`@\x81\x01\x82`@\x85`\x05\x1B\x84\x01\x01\x96\x01\x94_\x92[\x85\x84\x10a\x02jWPPPPPPP\x90V[\x90\x91\x92\x93\x80\x95\x96\x97`?\x19\x83\x82\x03\x01\x85R\x88Q\x82``\x81\x87\x85\x01\x93`\x01\x80`\xA0\x1B\x03\x81Q\x16\x86R\x01Q\x93\x87\x83\x82\x01R\x84Q\x80\x94R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x02\xC5WPPP\x90\x80`\x01\x92\x99\x01\x94\x01\x94\x01\x92\x95\x94\x93\x91\x90a\x02YV[\x82Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x84R\x89\x94\x93\x84\x01\x93\x90\x92\x01\x91`\x01\x91\x90\x91\x01\x90a\x02\xA5V[4a\0\xB5W_6`\x03\x19\x01\x12a\0\xB5W_\x80Q` a\x07\xEC\x839\x81Q\x91RTa\x03\x11\x81a\x07wV[a\x03\x1A\x82a\x06\x9FV[\x91_\x90_\x90[\x80\x82\x10a\x03{WPP_[\x81\x81\x10a\x03CW\x81\x83R`@Q\x80a\x01d\x85\x82a\x024V[\x80a\x03ea\x03^a\x03V`\x01\x94\x88a\x07<V[Qa\xFF\xFF\x16\x90V[a\xFF\xFF\x16\x90V[` a\x03q\x83\x87a\x07<V[Q\x01QR\x01a\x03+V[\x90\x91a\x03\x89a\x01{\x84a\x06\xE0V[a\x03\x95a\x01\x8D\x82a\x06\x12V[_\x80`\x01`\x01`\xA0\x1B\x03\x83\x16[\x85\x82\x10a\x04>W[PPa\x043W\x91a\x04\x17a\x04*\x92a\x03\xD8`\x01\x95a\x03\xC8\x85\x8Ba\x07<V[Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x04\x07a\x03\xE4\x87a\x06\x9FV[` \x90\x81a\x03\xF2\x87\x8Da\x07<V[Q\x01Ra\x03\xFF\x85\x8Ba\x07<V[Q\x01Qa\x07/V[`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x90RV[a\x01\xD8a\x04$\x82\x89a\x07<V[`\x01\x90RV[\x92[\x01\x90a\x03 V[PP\x91`\x01\x90a\x04,V[\x80a\x04ha\x02\x0Ea\x04Z\x85\x8D\x98\x9C\x9D\x9E\x97\x96\x9E\x9B\x99\x9A\x9Ba\x07<V[QQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x14a\x04\x83W`\x01\x80\x9A\x01\x91\x92\x99P\x97\x96\x92\x97\x95\x94\x93\x95a\x03\xA2V[PP\x96\x80a\x04\xBF\x85a\x04\x07\x8Ba\x04\xB9a\x03^a\x03V\x87\x8A\x9F\x9E\x9Aa\x04\xB0\x9C\x9E\x9D\x9Ca\x04\xE7\x9B` \x92a\x07<V[Q\x01Q\x94a\x07<V[\x90a\x07<V[a\x04\xDEa\x04\xD7a\x04\xD2a\x03V\x84\x8Da\x07<V[a\x07\xD8V[\x91\x8Aa\x07<V[\x90a\xFF\xFF\x16\x90RV[`\x01_\x80a\x03\xAAV[` \x90` `@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91_[\x82\x81\x10a\x05\x17WPPPP\x90V[\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x05\tV[4a\0\xB5W` 6`\x03\x19\x01\x12a\0\xB5W`\x01`\x01`\xA0\x1B\x03`\x045\x81\x81\x16\x90\x81\x90\x03a\0\xB5W_\x80Q` a\x07\xEC\x839\x81Q\x91RT\x91_\x90a\x05w\x84a\x06\x9FV[\x92_[\x85\x81\x10a\x05\x92W\x83\x85R`@Q\x80a\x01d\x87\x82a\x04\xF0V[a\x05\x9B\x81a\x06\xE0V[\x90T\x90`\x03\x1B\x1C`\xE0\x1B\x83a\x05\xAF\x82a\x06\x12V[T\x16\x83\x14a\x05\xC1W[P`\x01\x01a\x05zV[\x84a\x05\xE6\x91a\x05\xD3`\x01\x94\x97\x89a\x07<V[`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x90Ra\x07dV[\x93\x90a\x05\xB8V[4a\0\xB5W` `\x01`\x01`\xA0\x1B\x03a\x06\x08a\x01\x886a\0\xB9V[T\x16`@Q\x90\x81R\xF3[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16_R\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD2` R`@_ \x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q\x91\x90`\x1F\x01`\x1F\x19\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\x06\x82W`@RV[a\x06HV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\x82W`\x05\x1B` \x01\x90V[\x90a\x06\xB1a\x06\xAC\x83a\x06\x87V[a\x06\\V[\x82\x81R\x80\x92a\x06\xC2`\x1F\x19\x91a\x06\x87V[\x01\x90` 6\x91\x017V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90_\x80Q` a\x07\xEC\x839\x81Q\x91R\x80T\x83\x10\x15a\x07*W_R`\x1C\x82`\x03\x1C\x7F\xB6[\xEC\xA8\xB6\xFAx\x8B\xCB\x15(\xC2\xAB_M\xC6\xBC\x98\xE5\x89eP\xBA\xA0\x13\xD83\x0F\xAB\x0B\x86\xF4\x01\x92`\x02\x1B\x16\x90V[a\x06\xCCV[\x80Q\x15a\x07*W` \x01\x90V[\x80Q\x82\x10\x15a\x07*W` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x19\x81\x14a\x07rW`\x01\x01\x90V[a\x07PV[\x90a\x07\x84a\x06\xAC\x83a\x06\x87V[\x82\x81R\x80\x92a\x07\x95`\x1F\x19\x91a\x06\x87V[\x01_[\x81\x81\x10a\x07\xA4WPPPV[`@\x90\x81Q\x82\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x06\x82W` \x93R_\x81R\x82``\x81\x83\x01R\x82\x86\x01\x01R\x01a\x07\x98V[a\xFF\xFF\x80\x91\x16\x90\x81\x14a\x07rW`\x01\x01\x90V\xFE\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD3\xA2dipfsX\"\x12 \xE1U21\xA5\xF5/c\x8A\xF5\xAA\xD0\xA1\x7F\xF5,\x06P\xA1i)x3\xAA\xCEF\xA9\xEE\x01\xBB9;dsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static DIAMONDLOUPEFACET_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\0dW\x80cR\xEFk,\x14a\0_W\x80cz\x0E\xD6'\x14a\0ZW\x80c\xAD\xFC\xA1^\x14a\0UWc\xCD\xFF\xAC\xC6\x14a\0PW_\x80\xFD[a\x05\xEDV[a\x055V[a\x02\xE9V[a\x01 V[4a\0\xB5W`\x01`\x01`\xE0\x1B\x03\x19a\0{6a\0\xB9V[\x16_R\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD4` R`\xFF`@_ T\x16\x15\x15`\x80R` `\x80\xF3[_\x80\xFD[` \x90`\x03\x19\x01\x12a\0\xB5W`\x045`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\0\xB5W\x90V[` \x90` `@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91_[\x82\x81\x10a\x01\x03WPPPP\x90V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\0\xF5V[4a\0\xB5W_6`\x03\x19\x01\x12a\0\xB5W_\x80Q` a\x07\xEC\x839\x81Q\x91RTa\x01H\x81a\x06\x9FV[_\x80\x92[\x80\x84\x10a\x01hW\x81\x83R`@Q\x80a\x01d\x85\x82a\0\xDCV[\x03\x90\xF3[\x90a\x01\x9Aa\x01\x8Da\x01\x88a\x01{\x87a\x06\xE0V[\x90T\x90`\x03\x1B\x1C`\xE0\x1B\x90V[a\x06\x12V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[_`\x01`\x01`\xA0\x1B\x03\x82\x16\x81[\x84\x81\x10a\x01\xF1W[PPa\x01\xE7W\x81a\x01\xD8a\x01\xDD\x92a\x01\xC9`\x01\x95\x88a\x07<V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x07dV[\x93[\x01\x92\x90a\x01LV[P\x92`\x01\x90a\x01\xDFV[a\x02\x1Aa\x02\x0Ea\x02\x01\x83\x8Aa\x07<V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x82\x14a\x02(W`\x01\x01a\x01\xA7V[PPP`\x01_\x80a\x01\xAFV[` \x80\x82\x01\x90\x80\x83R\x83Q\x80\x92R`@\x92`@\x81\x01\x82`@\x85`\x05\x1B\x84\x01\x01\x96\x01\x94_\x92[\x85\x84\x10a\x02jWPPPPPPP\x90V[\x90\x91\x92\x93\x80\x95\x96\x97`?\x19\x83\x82\x03\x01\x85R\x88Q\x82``\x81\x87\x85\x01\x93`\x01\x80`\xA0\x1B\x03\x81Q\x16\x86R\x01Q\x93\x87\x83\x82\x01R\x84Q\x80\x94R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x02\xC5WPPP\x90\x80`\x01\x92\x99\x01\x94\x01\x94\x01\x92\x95\x94\x93\x91\x90a\x02YV[\x82Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x84R\x89\x94\x93\x84\x01\x93\x90\x92\x01\x91`\x01\x91\x90\x91\x01\x90a\x02\xA5V[4a\0\xB5W_6`\x03\x19\x01\x12a\0\xB5W_\x80Q` a\x07\xEC\x839\x81Q\x91RTa\x03\x11\x81a\x07wV[a\x03\x1A\x82a\x06\x9FV[\x91_\x90_\x90[\x80\x82\x10a\x03{WPP_[\x81\x81\x10a\x03CW\x81\x83R`@Q\x80a\x01d\x85\x82a\x024V[\x80a\x03ea\x03^a\x03V`\x01\x94\x88a\x07<V[Qa\xFF\xFF\x16\x90V[a\xFF\xFF\x16\x90V[` a\x03q\x83\x87a\x07<V[Q\x01QR\x01a\x03+V[\x90\x91a\x03\x89a\x01{\x84a\x06\xE0V[a\x03\x95a\x01\x8D\x82a\x06\x12V[_\x80`\x01`\x01`\xA0\x1B\x03\x83\x16[\x85\x82\x10a\x04>W[PPa\x043W\x91a\x04\x17a\x04*\x92a\x03\xD8`\x01\x95a\x03\xC8\x85\x8Ba\x07<V[Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x04\x07a\x03\xE4\x87a\x06\x9FV[` \x90\x81a\x03\xF2\x87\x8Da\x07<V[Q\x01Ra\x03\xFF\x85\x8Ba\x07<V[Q\x01Qa\x07/V[`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x90RV[a\x01\xD8a\x04$\x82\x89a\x07<V[`\x01\x90RV[\x92[\x01\x90a\x03 V[PP\x91`\x01\x90a\x04,V[\x80a\x04ha\x02\x0Ea\x04Z\x85\x8D\x98\x9C\x9D\x9E\x97\x96\x9E\x9B\x99\x9A\x9Ba\x07<V[QQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x14a\x04\x83W`\x01\x80\x9A\x01\x91\x92\x99P\x97\x96\x92\x97\x95\x94\x93\x95a\x03\xA2V[PP\x96\x80a\x04\xBF\x85a\x04\x07\x8Ba\x04\xB9a\x03^a\x03V\x87\x8A\x9F\x9E\x9Aa\x04\xB0\x9C\x9E\x9D\x9Ca\x04\xE7\x9B` \x92a\x07<V[Q\x01Q\x94a\x07<V[\x90a\x07<V[a\x04\xDEa\x04\xD7a\x04\xD2a\x03V\x84\x8Da\x07<V[a\x07\xD8V[\x91\x8Aa\x07<V[\x90a\xFF\xFF\x16\x90RV[`\x01_\x80a\x03\xAAV[` \x90` `@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91_[\x82\x81\x10a\x05\x17WPPPP\x90V[\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x05\tV[4a\0\xB5W` 6`\x03\x19\x01\x12a\0\xB5W`\x01`\x01`\xA0\x1B\x03`\x045\x81\x81\x16\x90\x81\x90\x03a\0\xB5W_\x80Q` a\x07\xEC\x839\x81Q\x91RT\x91_\x90a\x05w\x84a\x06\x9FV[\x92_[\x85\x81\x10a\x05\x92W\x83\x85R`@Q\x80a\x01d\x87\x82a\x04\xF0V[a\x05\x9B\x81a\x06\xE0V[\x90T\x90`\x03\x1B\x1C`\xE0\x1B\x83a\x05\xAF\x82a\x06\x12V[T\x16\x83\x14a\x05\xC1W[P`\x01\x01a\x05zV[\x84a\x05\xE6\x91a\x05\xD3`\x01\x94\x97\x89a\x07<V[`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x90Ra\x07dV[\x93\x90a\x05\xB8V[4a\0\xB5W` `\x01`\x01`\xA0\x1B\x03a\x06\x08a\x01\x886a\0\xB9V[T\x16`@Q\x90\x81R\xF3[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16_R\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD2` R`@_ \x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q\x91\x90`\x1F\x01`\x1F\x19\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\x06\x82W`@RV[a\x06HV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\x82W`\x05\x1B` \x01\x90V[\x90a\x06\xB1a\x06\xAC\x83a\x06\x87V[a\x06\\V[\x82\x81R\x80\x92a\x06\xC2`\x1F\x19\x91a\x06\x87V[\x01\x90` 6\x91\x017V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90_\x80Q` a\x07\xEC\x839\x81Q\x91R\x80T\x83\x10\x15a\x07*W_R`\x1C\x82`\x03\x1C\x7F\xB6[\xEC\xA8\xB6\xFAx\x8B\xCB\x15(\xC2\xAB_M\xC6\xBC\x98\xE5\x89eP\xBA\xA0\x13\xD83\x0F\xAB\x0B\x86\xF4\x01\x92`\x02\x1B\x16\x90V[a\x06\xCCV[\x80Q\x15a\x07*W` \x01\x90V[\x80Q\x82\x10\x15a\x07*W` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x19\x81\x14a\x07rW`\x01\x01\x90V[a\x07PV[\x90a\x07\x84a\x06\xAC\x83a\x06\x87V[\x82\x81R\x80\x92a\x07\x95`\x1F\x19\x91a\x06\x87V[\x01_[\x81\x81\x10a\x07\xA4WPPPV[`@\x90\x81Q\x82\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x06\x82W` \x93R_\x81R\x82``\x81\x83\x01R\x82\x86\x01\x01R\x01a\x07\x98V[a\xFF\xFF\x80\x91\x16\x90\x81\x14a\x07rW`\x01\x01\x90V\xFE\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD3\xA2dipfsX\"\x12 \xE1U21\xA5\xF5/c\x8A\xF5\xAA\xD0\xA1\x7F\xF5,\x06P\xA1i)x3\xAA\xCEF\xA9\xEE\x01\xBB9;dsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static DIAMONDLOUPEFACET_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DiamondLoupeFacet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DiamondLoupeFacet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DiamondLoupeFacet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DiamondLoupeFacet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DiamondLoupeFacet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DiamondLoupeFacet))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DiamondLoupeFacet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DIAMONDLOUPEFACET_ABI.clone(),
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
                DIAMONDLOUPEFACET_ABI.clone(),
                DIAMONDLOUPEFACET_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `facetAddress` (0xcdffacc6) function
        pub fn facet_address(
            &self,
            function_selector: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([205, 255, 172, 198], function_selector)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facetAddresses` (0x52ef6b2c) function
        pub fn facet_addresses(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([82, 239, 107, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facetFunctionSelectors` (0xadfca15e) function
        pub fn facet_function_selectors(
            &self,
            facet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 4]>> {
            self.0
                .method_hash([173, 252, 161, 94], facet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facets` (0x7a0ed627) function
        pub fn facets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Facet>> {
            self.0
                .method_hash([122, 14, 214, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DiamondLoupeFacet<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `facetAddress` function with signature `facetAddress(bytes4)` and selector `0xcdffacc6`
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
    #[ethcall(name = "facetAddress", abi = "facetAddress(bytes4)")]
    pub struct FacetAddressCall {
        pub function_selector: [u8; 4],
    }
    ///Container type for all input parameters for the `facetAddresses` function with signature `facetAddresses()` and selector `0x52ef6b2c`
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
    #[ethcall(name = "facetAddresses", abi = "facetAddresses()")]
    pub struct FacetAddressesCall;
    ///Container type for all input parameters for the `facetFunctionSelectors` function with signature `facetFunctionSelectors(address)` and selector `0xadfca15e`
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
    #[ethcall(name = "facetFunctionSelectors", abi = "facetFunctionSelectors(address)")]
    pub struct FacetFunctionSelectorsCall {
        pub facet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `facets` function with signature `facets()` and selector `0x7a0ed627`
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
    #[ethcall(name = "facets", abi = "facets()")]
    pub struct FacetsCall;
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DiamondLoupeFacetCalls {
        FacetAddress(FacetAddressCall),
        FacetAddresses(FacetAddressesCall),
        FacetFunctionSelectors(FacetFunctionSelectorsCall),
        Facets(FacetsCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for DiamondLoupeFacetCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FacetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FacetAddress(decoded));
            }
            if let Ok(decoded) = <FacetAddressesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FacetAddresses(decoded));
            }
            if let Ok(decoded) = <FacetFunctionSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FacetFunctionSelectors(decoded));
            }
            if let Ok(decoded) = <FacetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Facets(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DiamondLoupeFacetCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FacetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FacetAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FacetFunctionSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Facets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DiamondLoupeFacetCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FacetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetFunctionSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Facets(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FacetAddressCall> for DiamondLoupeFacetCalls {
        fn from(value: FacetAddressCall) -> Self {
            Self::FacetAddress(value)
        }
    }
    impl ::core::convert::From<FacetAddressesCall> for DiamondLoupeFacetCalls {
        fn from(value: FacetAddressesCall) -> Self {
            Self::FacetAddresses(value)
        }
    }
    impl ::core::convert::From<FacetFunctionSelectorsCall> for DiamondLoupeFacetCalls {
        fn from(value: FacetFunctionSelectorsCall) -> Self {
            Self::FacetFunctionSelectors(value)
        }
    }
    impl ::core::convert::From<FacetsCall> for DiamondLoupeFacetCalls {
        fn from(value: FacetsCall) -> Self {
            Self::Facets(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for DiamondLoupeFacetCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    ///Container type for all return fields from the `facetAddress` function with signature `facetAddress(bytes4)` and selector `0xcdffacc6`
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
    pub struct FacetAddressReturn {
        pub facet_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `facetAddresses` function with signature `facetAddresses()` and selector `0x52ef6b2c`
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
    pub struct FacetAddressesReturn {
        pub facet_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `facetFunctionSelectors` function with signature `facetFunctionSelectors(address)` and selector `0xadfca15e`
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
    pub struct FacetFunctionSelectorsReturn {
        pub facet_function_selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///Container type for all return fields from the `facets` function with signature `facets()` and selector `0x7a0ed627`
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
    pub struct FacetsReturn {
        pub facets: ::std::vec::Vec<Facet>,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///`Facet(address,bytes4[])`
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
    pub struct Facet {
        pub facet_address: ::ethers::core::types::Address,
        pub function_selectors: ::std::vec::Vec<[u8; 4]>,
    }
}
