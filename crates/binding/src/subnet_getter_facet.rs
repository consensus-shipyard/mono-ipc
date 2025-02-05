pub use subnet_getter_facet::*;
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
pub mod subnet_getter_facet {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getGateway"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getGateway"),
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
                    ::std::borrow::ToOwned::to_owned("getSubnetActorCheckpointerFacet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubnetActorCheckpointerFacet",
                            ),
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
                    ::std::borrow::ToOwned::to_owned(
                        "getSubnetActorCheckpointerSelectors",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubnetActorCheckpointerSelectors",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("getSubnetActorGetterFacet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubnetActorGetterFacet",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getSubnetActorGetterSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubnetActorGetterSelectors",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("getSubnetActorManagerFacet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubnetActorManagerFacet",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getSubnetActorManagerSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubnetActorManagerSelectors",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("getSubnetActorPauserFacet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubnetActorPauserFacet",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getSubnetActorPauserSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubnetActorPauserSelectors",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("getSubnetActorRewarderFacet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubnetActorRewarderFacet",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getSubnetActorRewarderSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubnetActorRewarderSelectors",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("getSubnetDeployedByNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSubnetDeployedByNonce",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
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
                    ::std::borrow::ToOwned::to_owned("getUserLastNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUserLastNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
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
                    ::std::borrow::ToOwned::to_owned("latestSubnetDeployed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestSubnetDeployed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
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
                    ::std::borrow::ToOwned::to_owned("updateReferenceSubnetContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateReferenceSubnetContract",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newGetterFacet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newManagerFacet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newSubnetGetterSelectors",
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newSubnetManagerSelectors",
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CannotFindSubnet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CannotFindSubnet"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("NotOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotOwner"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SUBNETGETTERFACET_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x0F%\x90\x81a\0\x1B\x829\xF3[_\x80\xFD\xFE`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x15W_\x80\xFD[`\xE0\x92_5\x84\x1C\x90\x81c\x03\x0F`Q\x14a\r\xB5WP\x80c\x0B\xE0a\x11\x14a\r\x8DW\x80c\x0FXI\xD1\x14a\x0CoW\x80c\x11c\xDC\xA5\x14a\x0B\xF8W\x80c\x1B\x07f\xC3\x14a\n\xDAW\x80cB\xBF<\xC1\x14a\n\xB3W\x80cMq\x15\x14\x14a\n\x8BW\x80cT\x0BZ\xD6\x14a\ncW\x80cT\xA4\xED\xDB\x14a\t2W\x80cb\xC9\xD7\xFB\x14a\t\x0BW\x80c\x89\xBB\xA2\x99\x14a\x07\xDAW\x80c\x96{\xA57\x14a\x05\xBCW\x80c\x986\xB7_\x14a\x05:W\x80c\xA3r\xBF0\x14a\x05\x12Wc\xA4m\x04M\x14a\0\xC6W_\x80\xFD[4a\x05\x0EW`\x806`\x03\x19\x01\x12a\x05\x0EWa\0\xDFa\x0E\x0EV[`\x01`\x01`\xA0\x1B\x03\x90`$5\x82\x81\x16\x91\x90\x82\x90\x03a\x05\x0EWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94`D5\x86\x81\x11a\x05\x0EWa\x01\x18\x906\x90\x83\x01a\x0EiV[\x96`d5\x81\x81\x11a\x05\x0EWa\x010\x906\x90\x85\x01a\x0EiV[\x96\x90\x94\x81\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD5T\x163\x03a\x04\xFEW\x16\x96\x87\x15a\x04\xEFW\x85\x15a\x04\xEFWPk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x94`\x01\x97\x86\x89T\x16\x17\x88U`\x02\x95\x86T\x16\x17\x85U\x80\x88\x11a\x04\xDCWh\x01\0\0\0\0\0\0\0\0\x91\x82\x89\x11a\x04\xC9W`\nT\x89`\nU\x80\x8A\x10a\x04FW[P`\n_R\x88`\x03\x1C_[\x81\x81\x10a\x03\xF8WP`\x07\x19\x99\x80\x8B\x16\x90\x03\x80a\x03\x8EW[PPP\x85\x11a\x03{W\x84\x11a\x03hWP`\x0BT\x83`\x0BU\x80\x84\x10a\x02\xE5W[P\x90`\x0B_R\x82`\x03\x1C\x94_[\x86\x81\x10a\x02\x84WP\x83\x16\x80\x84\x03\x93\x03a\x02\x1BW\0[\x94_\x93\x92\x91\x93\x95_\x91[\x83\x83\x10a\x02FWPPPPPP_\x80Q` a\x0E\xD0\x839\x81Q\x91R\x01U_\x80\xF3[\x90\x91\x92\x93` a\x02w\x87\x99a\x02[\x84\x99a\x0E\x9AV[\x85\x1C\x90\x87\x87\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x98\x01\x95\x94\x93\x01\x91\x90a\x02%V[\x85\x90_\x80[`\x08\x81\x10a\x02\xA9WP\x81_\x80Q` a\x0E\xD0\x839\x81Q\x91R\x01U\x01a\x02\x06V[\x95\x91\x92\x90a\x02\xD8` \x91a\x02\xBC\x85a\x0E\x9AV[\x8D\x1C\x90\x89\x89\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x92\x01\x95\x01\x90\x87\x92\x91a\x02\x89V[`\x07\x84\x01`\x03\x1C`\x07_\x80Q` a\x0E\xD0\x839\x81Q\x91R\x92\x01`\x03\x1C\x82\x01\x91`\x1C\x86\x86\x1B\x16\x80a\x03.W[P\x86\x91\x01[\x82\x81\x10a\x03#WPPa\x01\xF9V[_\x81U\x01\x85\x90a\x03\x15V[\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB8\x83\x01\x90\x81T\x90_\x19\x90` \x03`\x03\x1B\x1C\x16\x90U_a\x03\x10V[`A\x90cNH{q`\xE0\x1B_RR`$_\xFD[`A\x82cNH{q`\xE0\x1B_RR`$_\xFD[_\x92\x90\x83\x8B\x8A\x8F[\x84\x84\x10a\x03\xBBWPPPPPP_\x80Q` a\x0E\xB0\x839\x81Q\x91R\x01U_\x80\x80a\x01\xDAV[\x90\x85\x97\x84a\x03\xEB\x93a\x03\xD1` \x96\x97\x98\x99a\x0E\x9AV[\x90\x1C\x92\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x95\x01\x92\x91\x01\x8B\x8A\x8Fa\x03\x96V[\x89\x90_\x80\x8A\x8F[`\x08\x83\x10a\x04!WPPP\x81_\x80Q` a\x0E\xB0\x839\x81Q\x91R\x01U\x01a\x01\xC3V[\x90\x87\x94\x95\x93\x83a\x047\x93a\x03\xD1` \x96\x9Ba\x0E\x9AV[\x92\x01\x94\x01\x90\x8B\x92\x91\x8A\x8Fa\x03\xFFV[`\x07\x8A\x01`\x03\x1C`\x07_\x80Q` a\x0E\xB0\x839\x81Q\x91R\x92\x01`\x03\x1C\x82\x01\x91`\x1C\x8C\x8A\x1B\x16\x80a\x04\x8FW[P\x8A\x91\x01[\x82\x81\x10a\x04\x84WPPa\x01\xB8V[_\x81U\x01\x89\x90a\x04vV[\x7F\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA7\x83\x01\x90\x81T\x90_\x19\x90` \x03`\x03\x1B\x1C\x16\x90U_a\x04qV[`A\x84cNH{q`\xE0\x1B_RR`$_\xFD[`A\x83cNH{q`\xE0\x1B_RR`$_\xFD[Qc\x07\xA0CQ`\xE5\x1B\x81R\x83\x90\xFD[\x88Qc0\xCDtq`\xE0\x1B\x81R\x85\x90\xFD[_\x80\xFD[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW`\x02T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x05\x0EW\x80`\x03\x196\x01\x12a\x05\x0EWa\x05Sa\x0E\x0EV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x05\x0EW\x80\x15a\x05\xADW`\x01\x80`\xA0\x1B\x03\x80\x92\x16_R`\x13` R\x82_ \x90_R` R\x81_ T\x16\x90\x81\x15a\x05\x9EW` \x92PQ\x90\x81R\xF3[Qc'nt\xA7`\xE1\x1B\x81R\x90P\xFD[PPQc'nt\xA7`\xE1\x1B\x81R\xFD[P\x82\x904a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x80Q\x80\x92`\rT\x90\x81\x83R` \x80\x93\x01\x91`\r_R\x7F\xD7\xB6\x99\x01\x05q\x91\x01\xDA\xBE\xB7qD\xF2\xA38\\\x803\xAC\xD3\xAF\x97\xE9B:i^\x81\xAD\x1E\xB5\x84_\x91[\x83`\x07\x84\x01\x10a\x07lWT\x93\x83\x83\x10a\x07OW[P\x82\x82\x10a\x071W[\x82\x82\x10a\x07\x13W[\x82\x82\x10a\x06\xF5W[\x82\x82\x10a\x06\xD7W[\x82\x82\x10a\x06\xBBW[\x82\x82\x10a\x06\x9FW[P\x10a\x06\x8BW[P\x83\x90\x03`\x1F\x01`\x1F\x19\x16\x83\x01\x93\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x83\x85\x10\x17a\x03hWP\x82\x91\x82a\x06\x87\x92R\x82a\x0E$V[\x03\x90\xF3[`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R\x01\x80\x85a\x06UV[\x83\x81\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x06NV[\x83\x87\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x06FV[``\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x06>V[`\x80\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x066V[`\xA0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x06.V[`\xC0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x06&V[\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84\x89a\x06\x1DV[\x94`\x08\x91Pa\x01\0`\x01\x91\x87Tc\xFF\xFF\xFF\xFF`\xE0\x1B\x90\x81\x81\x8A\x1B\x16\x83R`\xC0\x82\x82\x82\x1B\x16\x8C\x85\x01R\x8C\x83\x83`\xA0\x92\x82\x82\x85\x1B\x16\x81\x89\x01R``\x83\x83`\x80\x92\x82\x82\x85\x1B\x16\x81\x8D\x01R\x1B\x16\x90\x89\x01R\x1B\x16\x90\x85\x01R\x82\x82\x8D\x1B\x16\x90\x84\x01R\x16\x87\x82\x01R\x01\x95\x01\x91\x01\x90\x85\x90a\x06\tV[P\x82\x904a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x80Q\x80\x92`\x0ET\x90\x81\x83R` \x80\x93\x01\x91`\x0E_R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x84_\x91[\x83`\x07\x84\x01\x10a\x08\x9DWT\x93\x83\x83\x10a\x07OWP\x82\x82\x10a\x071W\x82\x82\x10a\x07\x13W\x82\x82\x10a\x06\xF5W\x82\x82\x10a\x06\xD7W\x82\x82\x10a\x06\xBBW\x82\x82\x10a\x06\x9FWP\x10a\x06\x8BWP\x83\x90\x03`\x1F\x01`\x1F\x19\x16\x83\x01\x93\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x83\x85\x10\x17a\x03hWP\x82\x91\x82a\x06\x87\x92R\x82a\x0E$V[\x94`\x08\x91Pa\x01\0`\x01\x91\x87Tc\xFF\xFF\xFF\xFF`\xE0\x1B\x90\x81\x81\x8A\x1B\x16\x83R`\xC0\x82\x82\x82\x1B\x16\x8C\x85\x01R\x8C\x83\x83`\xA0\x92\x82\x82\x85\x1B\x16\x81\x89\x01R``\x83\x83`\x80\x92\x82\x82\x85\x1B\x16\x81\x8D\x01R\x1B\x16\x90\x89\x01R\x1B\x16\x90\x85\x01R\x82\x82\x8D\x1B\x16\x90\x84\x01R\x16\x87\x82\x01R\x01\x95\x01\x91\x01\x90\x85\x90a\x08'V[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x90T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P\x82\x904a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x80Q\x80\x92`\x0CT\x90\x81\x83R` \x80\x93\x01\x91`\x0C_R\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7\x84_\x91[\x83`\x07\x84\x01\x10a\t\xF5WT\x93\x83\x83\x10a\x07OWP\x82\x82\x10a\x071W\x82\x82\x10a\x07\x13W\x82\x82\x10a\x06\xF5W\x82\x82\x10a\x06\xD7W\x82\x82\x10a\x06\xBBW\x82\x82\x10a\x06\x9FWP\x10a\x06\x8BWP\x83\x90\x03`\x1F\x01`\x1F\x19\x16\x83\x01\x93\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x83\x85\x10\x17a\x03hWP\x82\x91\x82a\x06\x87\x92R\x82a\x0E$V[\x94`\x08\x91Pa\x01\0`\x01\x91\x87Tc\xFF\xFF\xFF\xFF`\xE0\x1B\x90\x81\x81\x8A\x1B\x16\x83R`\xC0\x82\x82\x82\x1B\x16\x8C\x85\x01R\x8C\x83\x83`\xA0\x92\x82\x82\x85\x1B\x16\x81\x89\x01R``\x83\x83`\x80\x92\x82\x82\x85\x1B\x16\x81\x8D\x01R\x1B\x16\x90\x89\x01R\x1B\x16\x90\x85\x01R\x82\x82\x8D\x1B\x16\x90\x84\x01R\x16\x87\x82\x01R\x01\x95\x01\x91\x01\x90\x85\x90a\t\x7FV[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW`\x03T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW`\x05T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW_T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P\x82\x904a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x80Q\x80\x92`\nT\x90\x81\x83R` \x80\x93\x01\x91`\n_R_\x80Q` a\x0E\xB0\x839\x81Q\x91R\x84_\x91[\x83`\x07\x84\x01\x10a\x0B\x8AWT\x93\x83\x83\x10a\x07OWP\x82\x82\x10a\x071W\x82\x82\x10a\x07\x13W\x82\x82\x10a\x06\xF5W\x82\x82\x10a\x06\xD7W\x82\x82\x10a\x06\xBBW\x82\x82\x10a\x06\x9FWP\x10a\x06\x8BWP\x83\x90\x03`\x1F\x01`\x1F\x19\x16\x83\x01\x93\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x83\x85\x10\x17a\x03hWP\x82\x91\x82a\x06\x87\x92R\x82a\x0E$V[\x94`\x08\x91Pa\x01\0`\x01\x91\x87Tc\xFF\xFF\xFF\xFF`\xE0\x1B\x90\x81\x81\x8A\x1B\x16\x83R`\xC0\x82\x82\x82\x1B\x16\x8C\x85\x01R\x8C\x83\x83`\xA0\x92\x82\x82\x85\x1B\x16\x81\x89\x01R``\x83\x83`\x80\x92\x82\x82\x85\x1B\x16\x81\x8D\x01R\x1B\x16\x90\x89\x01R\x1B\x16\x90\x85\x01R\x82\x82\x8D\x1B\x16\x90\x84\x01R\x16\x87\x82\x01R\x01\x95\x01\x91\x01\x90\x85\x90a\x0B\x14V[P4a\x05\x0EW` 6`\x03\x19\x01\x12a\x05\x0EW`\x01`\x01`\xA0\x1B\x03\x80a\x0C\x1Ba\x0E\x0EV[\x16\x80_R`\x14` Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83_ T\x16\x90\x81\x15a\x0C_W_R`\x13` R\x82_ \x90_R` R\x81_ T\x16\x90\x81\x15a\x05\x9EW` \x92PQ\x90\x81R\xF3[PPPQc'nt\xA7`\xE1\x1B\x81R\xFD[P\x82\x904a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x80Q\x80\x92`\x0BT\x90\x81\x83R` \x80\x93\x01\x91`\x0B_R_\x80Q` a\x0E\xD0\x839\x81Q\x91R\x84_\x91[\x83`\x07\x84\x01\x10a\r\x1FWT\x93\x83\x83\x10a\x07OWP\x82\x82\x10a\x071W\x82\x82\x10a\x07\x13W\x82\x82\x10a\x06\xF5W\x82\x82\x10a\x06\xD7W\x82\x82\x10a\x06\xBBW\x82\x82\x10a\x06\x9FWP\x10a\x06\x8BWP\x83\x90\x03`\x1F\x01`\x1F\x19\x16\x83\x01\x93\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x83\x85\x10\x17a\x03hWP\x82\x91\x82a\x06\x87\x92R\x82a\x0E$V[\x94`\x08\x91Pa\x01\0`\x01\x91\x87Tc\xFF\xFF\xFF\xFF`\xE0\x1B\x90\x81\x81\x8A\x1B\x16\x83R`\xC0\x82\x82\x82\x1B\x16\x8C\x85\x01R\x8C\x83\x83`\xA0\x92\x82\x82\x85\x1B\x16\x81\x89\x01R``\x83\x83`\x80\x92\x82\x82\x85\x1B\x16\x81\x8D\x01R\x1B\x16\x90\x89\x01R\x1B\x16\x90\x85\x01R\x82\x82\x8D\x1B\x16\x90\x84\x01R\x16\x87\x82\x01R\x01\x95\x01\x91\x01\x90\x85\x90a\x0C\xA9V[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW`\x01T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x91\x90P4a\x05\x0EW` 6`\x03\x19\x01\x12a\x05\x0EW`\x01`\x01`\xA0\x1B\x03a\r\xD9a\x0E\x0EV[\x16_R`\x14` Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_ T\x16\x91\x82\x15a\x0E\0W` \x83\x83Q\x90\x81R\xF3[c'nt\xA7`\xE1\x1B\x81R\x83\x90\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\x0EWV[` \x90` `@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91_[\x82\x81\x10a\x0EKWPPPP\x90V[\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x0E=V[\x91\x81`\x1F\x84\x01\x12\x15a\x05\x0EW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x05\x0EW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x05\x0EWV[5`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x05\x0EW\x90V\xFE\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA8\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\xA2dipfsX\"\x12 \xCC+z\xF4\xE6i\xDB\xAA\x7Fv\xE9V\xB7\x17\x1E\xAC13\xF9\x87\x07\x88*w\x99&g\rm\x1F\x7F3dsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static SUBNETGETTERFACET_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x15W_\x80\xFD[`\xE0\x92_5\x84\x1C\x90\x81c\x03\x0F`Q\x14a\r\xB5WP\x80c\x0B\xE0a\x11\x14a\r\x8DW\x80c\x0FXI\xD1\x14a\x0CoW\x80c\x11c\xDC\xA5\x14a\x0B\xF8W\x80c\x1B\x07f\xC3\x14a\n\xDAW\x80cB\xBF<\xC1\x14a\n\xB3W\x80cMq\x15\x14\x14a\n\x8BW\x80cT\x0BZ\xD6\x14a\ncW\x80cT\xA4\xED\xDB\x14a\t2W\x80cb\xC9\xD7\xFB\x14a\t\x0BW\x80c\x89\xBB\xA2\x99\x14a\x07\xDAW\x80c\x96{\xA57\x14a\x05\xBCW\x80c\x986\xB7_\x14a\x05:W\x80c\xA3r\xBF0\x14a\x05\x12Wc\xA4m\x04M\x14a\0\xC6W_\x80\xFD[4a\x05\x0EW`\x806`\x03\x19\x01\x12a\x05\x0EWa\0\xDFa\x0E\x0EV[`\x01`\x01`\xA0\x1B\x03\x90`$5\x82\x81\x16\x91\x90\x82\x90\x03a\x05\x0EWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94`D5\x86\x81\x11a\x05\x0EWa\x01\x18\x906\x90\x83\x01a\x0EiV[\x96`d5\x81\x81\x11a\x05\x0EWa\x010\x906\x90\x85\x01a\x0EiV[\x96\x90\x94\x81\x7F\x80n\x0C\xBB\x9F\xCE)k\xBC3jH\xF4+\xF1\xDB\xC6\x97\"\xD1\x8D\x90\xD6\xFEp[u\x82\xC2\xBBK\xD5T\x163\x03a\x04\xFEW\x16\x96\x87\x15a\x04\xEFW\x85\x15a\x04\xEFWPk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x94`\x01\x97\x86\x89T\x16\x17\x88U`\x02\x95\x86T\x16\x17\x85U\x80\x88\x11a\x04\xDCWh\x01\0\0\0\0\0\0\0\0\x91\x82\x89\x11a\x04\xC9W`\nT\x89`\nU\x80\x8A\x10a\x04FW[P`\n_R\x88`\x03\x1C_[\x81\x81\x10a\x03\xF8WP`\x07\x19\x99\x80\x8B\x16\x90\x03\x80a\x03\x8EW[PPP\x85\x11a\x03{W\x84\x11a\x03hWP`\x0BT\x83`\x0BU\x80\x84\x10a\x02\xE5W[P\x90`\x0B_R\x82`\x03\x1C\x94_[\x86\x81\x10a\x02\x84WP\x83\x16\x80\x84\x03\x93\x03a\x02\x1BW\0[\x94_\x93\x92\x91\x93\x95_\x91[\x83\x83\x10a\x02FWPPPPPP_\x80Q` a\x0E\xD0\x839\x81Q\x91R\x01U_\x80\xF3[\x90\x91\x92\x93` a\x02w\x87\x99a\x02[\x84\x99a\x0E\x9AV[\x85\x1C\x90\x87\x87\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x98\x01\x95\x94\x93\x01\x91\x90a\x02%V[\x85\x90_\x80[`\x08\x81\x10a\x02\xA9WP\x81_\x80Q` a\x0E\xD0\x839\x81Q\x91R\x01U\x01a\x02\x06V[\x95\x91\x92\x90a\x02\xD8` \x91a\x02\xBC\x85a\x0E\x9AV[\x8D\x1C\x90\x89\x89\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x92\x01\x95\x01\x90\x87\x92\x91a\x02\x89V[`\x07\x84\x01`\x03\x1C`\x07_\x80Q` a\x0E\xD0\x839\x81Q\x91R\x92\x01`\x03\x1C\x82\x01\x91`\x1C\x86\x86\x1B\x16\x80a\x03.W[P\x86\x91\x01[\x82\x81\x10a\x03#WPPa\x01\xF9V[_\x81U\x01\x85\x90a\x03\x15V[\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB8\x83\x01\x90\x81T\x90_\x19\x90` \x03`\x03\x1B\x1C\x16\x90U_a\x03\x10V[`A\x90cNH{q`\xE0\x1B_RR`$_\xFD[`A\x82cNH{q`\xE0\x1B_RR`$_\xFD[_\x92\x90\x83\x8B\x8A\x8F[\x84\x84\x10a\x03\xBBWPPPPPP_\x80Q` a\x0E\xB0\x839\x81Q\x91R\x01U_\x80\x80a\x01\xDAV[\x90\x85\x97\x84a\x03\xEB\x93a\x03\xD1` \x96\x97\x98\x99a\x0E\x9AV[\x90\x1C\x92\x1B`\x03\x1B\x91c\xFF\xFF\xFF\xFF\x80\x91\x16\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x95\x01\x92\x91\x01\x8B\x8A\x8Fa\x03\x96V[\x89\x90_\x80\x8A\x8F[`\x08\x83\x10a\x04!WPPP\x81_\x80Q` a\x0E\xB0\x839\x81Q\x91R\x01U\x01a\x01\xC3V[\x90\x87\x94\x95\x93\x83a\x047\x93a\x03\xD1` \x96\x9Ba\x0E\x9AV[\x92\x01\x94\x01\x90\x8B\x92\x91\x8A\x8Fa\x03\xFFV[`\x07\x8A\x01`\x03\x1C`\x07_\x80Q` a\x0E\xB0\x839\x81Q\x91R\x92\x01`\x03\x1C\x82\x01\x91`\x1C\x8C\x8A\x1B\x16\x80a\x04\x8FW[P\x8A\x91\x01[\x82\x81\x10a\x04\x84WPPa\x01\xB8V[_\x81U\x01\x89\x90a\x04vV[\x7F\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA7\x83\x01\x90\x81T\x90_\x19\x90` \x03`\x03\x1B\x1C\x16\x90U_a\x04qV[`A\x84cNH{q`\xE0\x1B_RR`$_\xFD[`A\x83cNH{q`\xE0\x1B_RR`$_\xFD[Qc\x07\xA0CQ`\xE5\x1B\x81R\x83\x90\xFD[\x88Qc0\xCDtq`\xE0\x1B\x81R\x85\x90\xFD[_\x80\xFD[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW`\x02T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x05\x0EW\x80`\x03\x196\x01\x12a\x05\x0EWa\x05Sa\x0E\x0EV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x05\x0EW\x80\x15a\x05\xADW`\x01\x80`\xA0\x1B\x03\x80\x92\x16_R`\x13` R\x82_ \x90_R` R\x81_ T\x16\x90\x81\x15a\x05\x9EW` \x92PQ\x90\x81R\xF3[Qc'nt\xA7`\xE1\x1B\x81R\x90P\xFD[PPQc'nt\xA7`\xE1\x1B\x81R\xFD[P\x82\x904a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x80Q\x80\x92`\rT\x90\x81\x83R` \x80\x93\x01\x91`\r_R\x7F\xD7\xB6\x99\x01\x05q\x91\x01\xDA\xBE\xB7qD\xF2\xA38\\\x803\xAC\xD3\xAF\x97\xE9B:i^\x81\xAD\x1E\xB5\x84_\x91[\x83`\x07\x84\x01\x10a\x07lWT\x93\x83\x83\x10a\x07OW[P\x82\x82\x10a\x071W[\x82\x82\x10a\x07\x13W[\x82\x82\x10a\x06\xF5W[\x82\x82\x10a\x06\xD7W[\x82\x82\x10a\x06\xBBW[\x82\x82\x10a\x06\x9FW[P\x10a\x06\x8BW[P\x83\x90\x03`\x1F\x01`\x1F\x19\x16\x83\x01\x93\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x83\x85\x10\x17a\x03hWP\x82\x91\x82a\x06\x87\x92R\x82a\x0E$V[\x03\x90\xF3[`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R\x01\x80\x85a\x06UV[\x83\x81\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x06NV[\x83\x87\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x06FV[``\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x06>V[`\x80\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x066V[`\xA0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x06.V[`\xC0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84a\x06&V[\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x90\x93\x01\x92`\x01\x01\x84\x89a\x06\x1DV[\x94`\x08\x91Pa\x01\0`\x01\x91\x87Tc\xFF\xFF\xFF\xFF`\xE0\x1B\x90\x81\x81\x8A\x1B\x16\x83R`\xC0\x82\x82\x82\x1B\x16\x8C\x85\x01R\x8C\x83\x83`\xA0\x92\x82\x82\x85\x1B\x16\x81\x89\x01R``\x83\x83`\x80\x92\x82\x82\x85\x1B\x16\x81\x8D\x01R\x1B\x16\x90\x89\x01R\x1B\x16\x90\x85\x01R\x82\x82\x8D\x1B\x16\x90\x84\x01R\x16\x87\x82\x01R\x01\x95\x01\x91\x01\x90\x85\x90a\x06\tV[P\x82\x904a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x80Q\x80\x92`\x0ET\x90\x81\x83R` \x80\x93\x01\x91`\x0E_R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x84_\x91[\x83`\x07\x84\x01\x10a\x08\x9DWT\x93\x83\x83\x10a\x07OWP\x82\x82\x10a\x071W\x82\x82\x10a\x07\x13W\x82\x82\x10a\x06\xF5W\x82\x82\x10a\x06\xD7W\x82\x82\x10a\x06\xBBW\x82\x82\x10a\x06\x9FWP\x10a\x06\x8BWP\x83\x90\x03`\x1F\x01`\x1F\x19\x16\x83\x01\x93\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x83\x85\x10\x17a\x03hWP\x82\x91\x82a\x06\x87\x92R\x82a\x0E$V[\x94`\x08\x91Pa\x01\0`\x01\x91\x87Tc\xFF\xFF\xFF\xFF`\xE0\x1B\x90\x81\x81\x8A\x1B\x16\x83R`\xC0\x82\x82\x82\x1B\x16\x8C\x85\x01R\x8C\x83\x83`\xA0\x92\x82\x82\x85\x1B\x16\x81\x89\x01R``\x83\x83`\x80\x92\x82\x82\x85\x1B\x16\x81\x8D\x01R\x1B\x16\x90\x89\x01R\x1B\x16\x90\x85\x01R\x82\x82\x8D\x1B\x16\x90\x84\x01R\x16\x87\x82\x01R\x01\x95\x01\x91\x01\x90\x85\x90a\x08'V[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x90T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P\x82\x904a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x80Q\x80\x92`\x0CT\x90\x81\x83R` \x80\x93\x01\x91`\x0C_R\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7\x84_\x91[\x83`\x07\x84\x01\x10a\t\xF5WT\x93\x83\x83\x10a\x07OWP\x82\x82\x10a\x071W\x82\x82\x10a\x07\x13W\x82\x82\x10a\x06\xF5W\x82\x82\x10a\x06\xD7W\x82\x82\x10a\x06\xBBW\x82\x82\x10a\x06\x9FWP\x10a\x06\x8BWP\x83\x90\x03`\x1F\x01`\x1F\x19\x16\x83\x01\x93\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x83\x85\x10\x17a\x03hWP\x82\x91\x82a\x06\x87\x92R\x82a\x0E$V[\x94`\x08\x91Pa\x01\0`\x01\x91\x87Tc\xFF\xFF\xFF\xFF`\xE0\x1B\x90\x81\x81\x8A\x1B\x16\x83R`\xC0\x82\x82\x82\x1B\x16\x8C\x85\x01R\x8C\x83\x83`\xA0\x92\x82\x82\x85\x1B\x16\x81\x89\x01R``\x83\x83`\x80\x92\x82\x82\x85\x1B\x16\x81\x8D\x01R\x1B\x16\x90\x89\x01R\x1B\x16\x90\x85\x01R\x82\x82\x8D\x1B\x16\x90\x84\x01R\x16\x87\x82\x01R\x01\x95\x01\x91\x01\x90\x85\x90a\t\x7FV[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW`\x03T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW`\x05T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW_T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P\x82\x904a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x80Q\x80\x92`\nT\x90\x81\x83R` \x80\x93\x01\x91`\n_R_\x80Q` a\x0E\xB0\x839\x81Q\x91R\x84_\x91[\x83`\x07\x84\x01\x10a\x0B\x8AWT\x93\x83\x83\x10a\x07OWP\x82\x82\x10a\x071W\x82\x82\x10a\x07\x13W\x82\x82\x10a\x06\xF5W\x82\x82\x10a\x06\xD7W\x82\x82\x10a\x06\xBBW\x82\x82\x10a\x06\x9FWP\x10a\x06\x8BWP\x83\x90\x03`\x1F\x01`\x1F\x19\x16\x83\x01\x93\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x83\x85\x10\x17a\x03hWP\x82\x91\x82a\x06\x87\x92R\x82a\x0E$V[\x94`\x08\x91Pa\x01\0`\x01\x91\x87Tc\xFF\xFF\xFF\xFF`\xE0\x1B\x90\x81\x81\x8A\x1B\x16\x83R`\xC0\x82\x82\x82\x1B\x16\x8C\x85\x01R\x8C\x83\x83`\xA0\x92\x82\x82\x85\x1B\x16\x81\x89\x01R``\x83\x83`\x80\x92\x82\x82\x85\x1B\x16\x81\x8D\x01R\x1B\x16\x90\x89\x01R\x1B\x16\x90\x85\x01R\x82\x82\x8D\x1B\x16\x90\x84\x01R\x16\x87\x82\x01R\x01\x95\x01\x91\x01\x90\x85\x90a\x0B\x14V[P4a\x05\x0EW` 6`\x03\x19\x01\x12a\x05\x0EW`\x01`\x01`\xA0\x1B\x03\x80a\x0C\x1Ba\x0E\x0EV[\x16\x80_R`\x14` Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83_ T\x16\x90\x81\x15a\x0C_W_R`\x13` R\x82_ \x90_R` R\x81_ T\x16\x90\x81\x15a\x05\x9EW` \x92PQ\x90\x81R\xF3[PPPQc'nt\xA7`\xE1\x1B\x81R\xFD[P\x82\x904a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW\x80Q\x80\x92`\x0BT\x90\x81\x83R` \x80\x93\x01\x91`\x0B_R_\x80Q` a\x0E\xD0\x839\x81Q\x91R\x84_\x91[\x83`\x07\x84\x01\x10a\r\x1FWT\x93\x83\x83\x10a\x07OWP\x82\x82\x10a\x071W\x82\x82\x10a\x07\x13W\x82\x82\x10a\x06\xF5W\x82\x82\x10a\x06\xD7W\x82\x82\x10a\x06\xBBW\x82\x82\x10a\x06\x9FWP\x10a\x06\x8BWP\x83\x90\x03`\x1F\x01`\x1F\x19\x16\x83\x01\x93\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x83\x85\x10\x17a\x03hWP\x82\x91\x82a\x06\x87\x92R\x82a\x0E$V[\x94`\x08\x91Pa\x01\0`\x01\x91\x87Tc\xFF\xFF\xFF\xFF`\xE0\x1B\x90\x81\x81\x8A\x1B\x16\x83R`\xC0\x82\x82\x82\x1B\x16\x8C\x85\x01R\x8C\x83\x83`\xA0\x92\x82\x82\x85\x1B\x16\x81\x89\x01R``\x83\x83`\x80\x92\x82\x82\x85\x1B\x16\x81\x8D\x01R\x1B\x16\x90\x89\x01R\x1B\x16\x90\x85\x01R\x82\x82\x8D\x1B\x16\x90\x84\x01R\x16\x87\x82\x01R\x01\x95\x01\x91\x01\x90\x85\x90a\x0C\xA9V[P4a\x05\x0EW_6`\x03\x19\x01\x12a\x05\x0EW`\x01T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x91\x90P4a\x05\x0EW` 6`\x03\x19\x01\x12a\x05\x0EW`\x01`\x01`\xA0\x1B\x03a\r\xD9a\x0E\x0EV[\x16_R`\x14` Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_ T\x16\x91\x82\x15a\x0E\0W` \x83\x83Q\x90\x81R\xF3[c'nt\xA7`\xE1\x1B\x81R\x83\x90\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\x0EWV[` \x90` `@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91_[\x82\x81\x10a\x0EKWPPPP\x90V[\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x0E=V[\x91\x81`\x1F\x84\x01\x12\x15a\x05\x0EW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x05\x0EW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x05\x0EWV[5`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x05\x0EW\x90V\xFE\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA8\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\xA2dipfsX\"\x12 \xCC+z\xF4\xE6i\xDB\xAA\x7Fv\xE9V\xB7\x17\x1E\xAC13\xF9\x87\x07\x88*w\x99&g\rm\x1F\x7F3dsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static SUBNETGETTERFACET_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SubnetGetterFacet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SubnetGetterFacet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SubnetGetterFacet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SubnetGetterFacet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SubnetGetterFacet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SubnetGetterFacet))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SubnetGetterFacet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SUBNETGETTERFACET_ABI.clone(),
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
                SUBNETGETTERFACET_ABI.clone(),
                SUBNETGETTERFACET_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getGateway` (0x42bf3cc1) function
        pub fn get_gateway(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([66, 191, 60, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetActorCheckpointerFacet` (0x62c9d7fb) function
        pub fn get_subnet_actor_checkpointer_facet(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([98, 201, 215, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetActorCheckpointerSelectors` (0x967ba537) function
        pub fn get_subnet_actor_checkpointer_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 4]>> {
            self.0
                .method_hash([150, 123, 165, 55], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetActorGetterFacet` (0x0be06111) function
        pub fn get_subnet_actor_getter_facet(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([11, 224, 97, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetActorGetterSelectors` (0x1b0766c3) function
        pub fn get_subnet_actor_getter_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 4]>> {
            self.0
                .method_hash([27, 7, 102, 195], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetActorManagerFacet` (0xa372bf30) function
        pub fn get_subnet_actor_manager_facet(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([163, 114, 191, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetActorManagerSelectors` (0x0f5849d1) function
        pub fn get_subnet_actor_manager_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 4]>> {
            self.0
                .method_hash([15, 88, 73, 209], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetActorPauserFacet` (0x4d711514) function
        pub fn get_subnet_actor_pauser_facet(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([77, 113, 21, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetActorPauserSelectors` (0x89bba299) function
        pub fn get_subnet_actor_pauser_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 4]>> {
            self.0
                .method_hash([137, 187, 162, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetActorRewarderFacet` (0x540b5ad6) function
        pub fn get_subnet_actor_rewarder_facet(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([84, 11, 90, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetActorRewarderSelectors` (0x54a4eddb) function
        pub fn get_subnet_actor_rewarder_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 4]>> {
            self.0
                .method_hash([84, 164, 237, 219], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetDeployedByNonce` (0x9836b75f) function
        pub fn get_subnet_deployed_by_nonce(
            &self,
            owner: ::ethers::core::types::Address,
            nonce: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([152, 54, 183, 95], (owner, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserLastNonce` (0x030f6051) function
        pub fn get_user_last_nonce(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([3, 15, 96, 81], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestSubnetDeployed` (0x1163dca5) function
        pub fn latest_subnet_deployed(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([17, 99, 220, 165], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateReferenceSubnetContract` (0xa46d044d) function
        pub fn update_reference_subnet_contract(
            &self,
            new_getter_facet: ::ethers::core::types::Address,
            new_manager_facet: ::ethers::core::types::Address,
            new_subnet_getter_selectors: ::std::vec::Vec<[u8; 4]>,
            new_subnet_manager_selectors: ::std::vec::Vec<[u8; 4]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [164, 109, 4, 77],
                    (
                        new_getter_facet,
                        new_manager_facet,
                        new_subnet_getter_selectors,
                        new_subnet_manager_selectors,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SubnetGetterFacet<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CannotFindSubnet` with signature `CannotFindSubnet()` and selector `0x4edce94e`
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
    #[etherror(name = "CannotFindSubnet", abi = "CannotFindSubnet()")]
    pub struct CannotFindSubnet;
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
    ///Custom Error type `NotOwner` with signature `NotOwner()` and selector `0x30cd7471`
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
    #[etherror(name = "NotOwner", abi = "NotOwner()")]
    pub struct NotOwner;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SubnetGetterFacetErrors {
        CannotFindSubnet(CannotFindSubnet),
        FacetCannotBeZero(FacetCannotBeZero),
        NotOwner(NotOwner),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SubnetGetterFacetErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <CannotFindSubnet as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotFindSubnet(decoded));
            }
            if let Ok(decoded) = <FacetCannotBeZero as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FacetCannotBeZero(decoded));
            }
            if let Ok(decoded) = <NotOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SubnetGetterFacetErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CannotFindSubnet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FacetCannotBeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SubnetGetterFacetErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CannotFindSubnet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FacetCannotBeZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotOwner as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SubnetGetterFacetErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CannotFindSubnet(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetCannotBeZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SubnetGetterFacetErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CannotFindSubnet> for SubnetGetterFacetErrors {
        fn from(value: CannotFindSubnet) -> Self {
            Self::CannotFindSubnet(value)
        }
    }
    impl ::core::convert::From<FacetCannotBeZero> for SubnetGetterFacetErrors {
        fn from(value: FacetCannotBeZero) -> Self {
            Self::FacetCannotBeZero(value)
        }
    }
    impl ::core::convert::From<NotOwner> for SubnetGetterFacetErrors {
        fn from(value: NotOwner) -> Self {
            Self::NotOwner(value)
        }
    }
    ///Container type for all input parameters for the `getGateway` function with signature `getGateway()` and selector `0x42bf3cc1`
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
    #[ethcall(name = "getGateway", abi = "getGateway()")]
    pub struct GetGatewayCall;
    ///Container type for all input parameters for the `getSubnetActorCheckpointerFacet` function with signature `getSubnetActorCheckpointerFacet()` and selector `0x62c9d7fb`
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
        name = "getSubnetActorCheckpointerFacet",
        abi = "getSubnetActorCheckpointerFacet()"
    )]
    pub struct GetSubnetActorCheckpointerFacetCall;
    ///Container type for all input parameters for the `getSubnetActorCheckpointerSelectors` function with signature `getSubnetActorCheckpointerSelectors()` and selector `0x967ba537`
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
        name = "getSubnetActorCheckpointerSelectors",
        abi = "getSubnetActorCheckpointerSelectors()"
    )]
    pub struct GetSubnetActorCheckpointerSelectorsCall;
    ///Container type for all input parameters for the `getSubnetActorGetterFacet` function with signature `getSubnetActorGetterFacet()` and selector `0x0be06111`
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
    #[ethcall(name = "getSubnetActorGetterFacet", abi = "getSubnetActorGetterFacet()")]
    pub struct GetSubnetActorGetterFacetCall;
    ///Container type for all input parameters for the `getSubnetActorGetterSelectors` function with signature `getSubnetActorGetterSelectors()` and selector `0x1b0766c3`
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
        name = "getSubnetActorGetterSelectors",
        abi = "getSubnetActorGetterSelectors()"
    )]
    pub struct GetSubnetActorGetterSelectorsCall;
    ///Container type for all input parameters for the `getSubnetActorManagerFacet` function with signature `getSubnetActorManagerFacet()` and selector `0xa372bf30`
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
    #[ethcall(name = "getSubnetActorManagerFacet", abi = "getSubnetActorManagerFacet()")]
    pub struct GetSubnetActorManagerFacetCall;
    ///Container type for all input parameters for the `getSubnetActorManagerSelectors` function with signature `getSubnetActorManagerSelectors()` and selector `0x0f5849d1`
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
        name = "getSubnetActorManagerSelectors",
        abi = "getSubnetActorManagerSelectors()"
    )]
    pub struct GetSubnetActorManagerSelectorsCall;
    ///Container type for all input parameters for the `getSubnetActorPauserFacet` function with signature `getSubnetActorPauserFacet()` and selector `0x4d711514`
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
    #[ethcall(name = "getSubnetActorPauserFacet", abi = "getSubnetActorPauserFacet()")]
    pub struct GetSubnetActorPauserFacetCall;
    ///Container type for all input parameters for the `getSubnetActorPauserSelectors` function with signature `getSubnetActorPauserSelectors()` and selector `0x89bba299`
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
        name = "getSubnetActorPauserSelectors",
        abi = "getSubnetActorPauserSelectors()"
    )]
    pub struct GetSubnetActorPauserSelectorsCall;
    ///Container type for all input parameters for the `getSubnetActorRewarderFacet` function with signature `getSubnetActorRewarderFacet()` and selector `0x540b5ad6`
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
        name = "getSubnetActorRewarderFacet",
        abi = "getSubnetActorRewarderFacet()"
    )]
    pub struct GetSubnetActorRewarderFacetCall;
    ///Container type for all input parameters for the `getSubnetActorRewarderSelectors` function with signature `getSubnetActorRewarderSelectors()` and selector `0x54a4eddb`
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
        name = "getSubnetActorRewarderSelectors",
        abi = "getSubnetActorRewarderSelectors()"
    )]
    pub struct GetSubnetActorRewarderSelectorsCall;
    ///Container type for all input parameters for the `getSubnetDeployedByNonce` function with signature `getSubnetDeployedByNonce(address,uint64)` and selector `0x9836b75f`
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
        name = "getSubnetDeployedByNonce",
        abi = "getSubnetDeployedByNonce(address,uint64)"
    )]
    pub struct GetSubnetDeployedByNonceCall {
        pub owner: ::ethers::core::types::Address,
        pub nonce: u64,
    }
    ///Container type for all input parameters for the `getUserLastNonce` function with signature `getUserLastNonce(address)` and selector `0x030f6051`
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
    #[ethcall(name = "getUserLastNonce", abi = "getUserLastNonce(address)")]
    pub struct GetUserLastNonceCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `latestSubnetDeployed` function with signature `latestSubnetDeployed(address)` and selector `0x1163dca5`
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
    #[ethcall(name = "latestSubnetDeployed", abi = "latestSubnetDeployed(address)")]
    pub struct LatestSubnetDeployedCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateReferenceSubnetContract` function with signature `updateReferenceSubnetContract(address,address,bytes4[],bytes4[])` and selector `0xa46d044d`
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
        name = "updateReferenceSubnetContract",
        abi = "updateReferenceSubnetContract(address,address,bytes4[],bytes4[])"
    )]
    pub struct UpdateReferenceSubnetContractCall {
        pub new_getter_facet: ::ethers::core::types::Address,
        pub new_manager_facet: ::ethers::core::types::Address,
        pub new_subnet_getter_selectors: ::std::vec::Vec<[u8; 4]>,
        pub new_subnet_manager_selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SubnetGetterFacetCalls {
        GetGateway(GetGatewayCall),
        GetSubnetActorCheckpointerFacet(GetSubnetActorCheckpointerFacetCall),
        GetSubnetActorCheckpointerSelectors(GetSubnetActorCheckpointerSelectorsCall),
        GetSubnetActorGetterFacet(GetSubnetActorGetterFacetCall),
        GetSubnetActorGetterSelectors(GetSubnetActorGetterSelectorsCall),
        GetSubnetActorManagerFacet(GetSubnetActorManagerFacetCall),
        GetSubnetActorManagerSelectors(GetSubnetActorManagerSelectorsCall),
        GetSubnetActorPauserFacet(GetSubnetActorPauserFacetCall),
        GetSubnetActorPauserSelectors(GetSubnetActorPauserSelectorsCall),
        GetSubnetActorRewarderFacet(GetSubnetActorRewarderFacetCall),
        GetSubnetActorRewarderSelectors(GetSubnetActorRewarderSelectorsCall),
        GetSubnetDeployedByNonce(GetSubnetDeployedByNonceCall),
        GetUserLastNonce(GetUserLastNonceCall),
        LatestSubnetDeployed(LatestSubnetDeployedCall),
        UpdateReferenceSubnetContract(UpdateReferenceSubnetContractCall),
    }
    impl ::ethers::core::abi::AbiDecode for SubnetGetterFacetCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetGatewayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetGateway(decoded));
            }
            if let Ok(decoded) = <GetSubnetActorCheckpointerFacetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubnetActorCheckpointerFacet(decoded));
            }
            if let Ok(decoded) = <GetSubnetActorCheckpointerSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubnetActorCheckpointerSelectors(decoded));
            }
            if let Ok(decoded) = <GetSubnetActorGetterFacetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubnetActorGetterFacet(decoded));
            }
            if let Ok(decoded) = <GetSubnetActorGetterSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubnetActorGetterSelectors(decoded));
            }
            if let Ok(decoded) = <GetSubnetActorManagerFacetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubnetActorManagerFacet(decoded));
            }
            if let Ok(decoded) = <GetSubnetActorManagerSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubnetActorManagerSelectors(decoded));
            }
            if let Ok(decoded) = <GetSubnetActorPauserFacetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubnetActorPauserFacet(decoded));
            }
            if let Ok(decoded) = <GetSubnetActorPauserSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubnetActorPauserSelectors(decoded));
            }
            if let Ok(decoded) = <GetSubnetActorRewarderFacetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubnetActorRewarderFacet(decoded));
            }
            if let Ok(decoded) = <GetSubnetActorRewarderSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubnetActorRewarderSelectors(decoded));
            }
            if let Ok(decoded) = <GetSubnetDeployedByNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubnetDeployedByNonce(decoded));
            }
            if let Ok(decoded) = <GetUserLastNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUserLastNonce(decoded));
            }
            if let Ok(decoded) = <LatestSubnetDeployedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestSubnetDeployed(decoded));
            }
            if let Ok(decoded) = <UpdateReferenceSubnetContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateReferenceSubnetContract(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SubnetGetterFacetCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetGateway(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetActorCheckpointerFacet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetActorCheckpointerSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetActorGetterFacet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetActorGetterSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetActorManagerFacet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetActorManagerSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetActorPauserFacet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetActorPauserSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetActorRewarderFacet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetActorRewarderSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetDeployedByNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserLastNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestSubnetDeployed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateReferenceSubnetContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SubnetGetterFacetCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetGateway(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubnetActorCheckpointerFacet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubnetActorCheckpointerSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubnetActorGetterFacet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubnetActorGetterSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubnetActorManagerFacet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubnetActorManagerSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubnetActorPauserFacet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubnetActorPauserSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubnetActorRewarderFacet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubnetActorRewarderSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubnetDeployedByNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUserLastNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestSubnetDeployed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateReferenceSubnetContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetGatewayCall> for SubnetGetterFacetCalls {
        fn from(value: GetGatewayCall) -> Self {
            Self::GetGateway(value)
        }
    }
    impl ::core::convert::From<GetSubnetActorCheckpointerFacetCall>
    for SubnetGetterFacetCalls {
        fn from(value: GetSubnetActorCheckpointerFacetCall) -> Self {
            Self::GetSubnetActorCheckpointerFacet(value)
        }
    }
    impl ::core::convert::From<GetSubnetActorCheckpointerSelectorsCall>
    for SubnetGetterFacetCalls {
        fn from(value: GetSubnetActorCheckpointerSelectorsCall) -> Self {
            Self::GetSubnetActorCheckpointerSelectors(value)
        }
    }
    impl ::core::convert::From<GetSubnetActorGetterFacetCall>
    for SubnetGetterFacetCalls {
        fn from(value: GetSubnetActorGetterFacetCall) -> Self {
            Self::GetSubnetActorGetterFacet(value)
        }
    }
    impl ::core::convert::From<GetSubnetActorGetterSelectorsCall>
    for SubnetGetterFacetCalls {
        fn from(value: GetSubnetActorGetterSelectorsCall) -> Self {
            Self::GetSubnetActorGetterSelectors(value)
        }
    }
    impl ::core::convert::From<GetSubnetActorManagerFacetCall>
    for SubnetGetterFacetCalls {
        fn from(value: GetSubnetActorManagerFacetCall) -> Self {
            Self::GetSubnetActorManagerFacet(value)
        }
    }
    impl ::core::convert::From<GetSubnetActorManagerSelectorsCall>
    for SubnetGetterFacetCalls {
        fn from(value: GetSubnetActorManagerSelectorsCall) -> Self {
            Self::GetSubnetActorManagerSelectors(value)
        }
    }
    impl ::core::convert::From<GetSubnetActorPauserFacetCall>
    for SubnetGetterFacetCalls {
        fn from(value: GetSubnetActorPauserFacetCall) -> Self {
            Self::GetSubnetActorPauserFacet(value)
        }
    }
    impl ::core::convert::From<GetSubnetActorPauserSelectorsCall>
    for SubnetGetterFacetCalls {
        fn from(value: GetSubnetActorPauserSelectorsCall) -> Self {
            Self::GetSubnetActorPauserSelectors(value)
        }
    }
    impl ::core::convert::From<GetSubnetActorRewarderFacetCall>
    for SubnetGetterFacetCalls {
        fn from(value: GetSubnetActorRewarderFacetCall) -> Self {
            Self::GetSubnetActorRewarderFacet(value)
        }
    }
    impl ::core::convert::From<GetSubnetActorRewarderSelectorsCall>
    for SubnetGetterFacetCalls {
        fn from(value: GetSubnetActorRewarderSelectorsCall) -> Self {
            Self::GetSubnetActorRewarderSelectors(value)
        }
    }
    impl ::core::convert::From<GetSubnetDeployedByNonceCall> for SubnetGetterFacetCalls {
        fn from(value: GetSubnetDeployedByNonceCall) -> Self {
            Self::GetSubnetDeployedByNonce(value)
        }
    }
    impl ::core::convert::From<GetUserLastNonceCall> for SubnetGetterFacetCalls {
        fn from(value: GetUserLastNonceCall) -> Self {
            Self::GetUserLastNonce(value)
        }
    }
    impl ::core::convert::From<LatestSubnetDeployedCall> for SubnetGetterFacetCalls {
        fn from(value: LatestSubnetDeployedCall) -> Self {
            Self::LatestSubnetDeployed(value)
        }
    }
    impl ::core::convert::From<UpdateReferenceSubnetContractCall>
    for SubnetGetterFacetCalls {
        fn from(value: UpdateReferenceSubnetContractCall) -> Self {
            Self::UpdateReferenceSubnetContract(value)
        }
    }
    ///Container type for all return fields from the `getGateway` function with signature `getGateway()` and selector `0x42bf3cc1`
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
    pub struct GetGatewayReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSubnetActorCheckpointerFacet` function with signature `getSubnetActorCheckpointerFacet()` and selector `0x62c9d7fb`
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
    pub struct GetSubnetActorCheckpointerFacetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSubnetActorCheckpointerSelectors` function with signature `getSubnetActorCheckpointerSelectors()` and selector `0x967ba537`
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
    pub struct GetSubnetActorCheckpointerSelectorsReturn(pub ::std::vec::Vec<[u8; 4]>);
    ///Container type for all return fields from the `getSubnetActorGetterFacet` function with signature `getSubnetActorGetterFacet()` and selector `0x0be06111`
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
    pub struct GetSubnetActorGetterFacetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSubnetActorGetterSelectors` function with signature `getSubnetActorGetterSelectors()` and selector `0x1b0766c3`
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
    pub struct GetSubnetActorGetterSelectorsReturn(pub ::std::vec::Vec<[u8; 4]>);
    ///Container type for all return fields from the `getSubnetActorManagerFacet` function with signature `getSubnetActorManagerFacet()` and selector `0xa372bf30`
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
    pub struct GetSubnetActorManagerFacetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSubnetActorManagerSelectors` function with signature `getSubnetActorManagerSelectors()` and selector `0x0f5849d1`
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
    pub struct GetSubnetActorManagerSelectorsReturn(pub ::std::vec::Vec<[u8; 4]>);
    ///Container type for all return fields from the `getSubnetActorPauserFacet` function with signature `getSubnetActorPauserFacet()` and selector `0x4d711514`
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
    pub struct GetSubnetActorPauserFacetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSubnetActorPauserSelectors` function with signature `getSubnetActorPauserSelectors()` and selector `0x89bba299`
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
    pub struct GetSubnetActorPauserSelectorsReturn(pub ::std::vec::Vec<[u8; 4]>);
    ///Container type for all return fields from the `getSubnetActorRewarderFacet` function with signature `getSubnetActorRewarderFacet()` and selector `0x540b5ad6`
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
    pub struct GetSubnetActorRewarderFacetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSubnetActorRewarderSelectors` function with signature `getSubnetActorRewarderSelectors()` and selector `0x54a4eddb`
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
    pub struct GetSubnetActorRewarderSelectorsReturn(pub ::std::vec::Vec<[u8; 4]>);
    ///Container type for all return fields from the `getSubnetDeployedByNonce` function with signature `getSubnetDeployedByNonce(address,uint64)` and selector `0x9836b75f`
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
    pub struct GetSubnetDeployedByNonceReturn {
        pub subnet: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getUserLastNonce` function with signature `getUserLastNonce(address)` and selector `0x030f6051`
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
    pub struct GetUserLastNonceReturn {
        pub nonce: u64,
    }
    ///Container type for all return fields from the `latestSubnetDeployed` function with signature `latestSubnetDeployed(address)` and selector `0x1163dca5`
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
    pub struct LatestSubnetDeployedReturn {
        pub subnet: ::ethers::core::types::Address,
    }
}
