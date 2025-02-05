// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//! Deserialization utils for config mod.

use crate::config::Subnet;
use anyhow::anyhow;
use fvm_shared::address::Address;
use http::HeaderValue;
use ipc_api::subnet_id::SubnetID;
use ipc_types::EthAddress;
use serde::de::{Error, SeqAccess};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::str::FromStr;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin};
use url::Url;

/// A serde deserialization method to deserialize a hashmap of subnets with subnet id as key and
/// Subnet struct as value from a vec of subnets
pub(crate) fn deserialize_subnets_from_vec<'de, D>(
    deserializer: D,
) -> anyhow::Result<HashMap<SubnetID, Subnet>, D::Error>
where
    D: Deserializer<'de>,
{
    let subnets = <Vec<Subnet>>::deserialize(deserializer)?;

    let mut hashmap = HashMap::new();
    for subnet in subnets {
        hashmap.insert(subnet.id.clone(), subnet);
    }
    Ok(hashmap)
}

/// A serde deserialization method to deserialize an address from i64
pub(crate) fn deserialize_address_from_str<'de, D>(
    deserializer: D,
) -> anyhow::Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = Address;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("an string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Address::from_str(v).map_err(E::custom)
        }
    }
    deserializer.deserialize_str(Visitor)
}

/// A serde deserialization method to deserialize an eth address from string, i.e. "0x...."
pub fn deserialize_eth_address_from_str<'de, D>(
    deserializer: D,
) -> anyhow::Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = Address;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            eth_addr_str_to_address(v).map_err(E::custom)
        }
    }
    deserializer.deserialize_str(Visitor)
}

/// A serde deserialization method to deserialize a subnet path string into a [`SubnetID`].
pub(crate) fn deserialize_subnet_id<'de, D>(deserializer: D) -> anyhow::Result<SubnetID, D::Error>
where
    D: Deserializer<'de>,
{
    struct SubnetIDVisitor;
    impl<'de> serde::de::Visitor<'de> for SubnetIDVisitor {
        type Value = SubnetID;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            SubnetID::from_str(v).map_err(E::custom)
        }
    }
    deserializer.deserialize_str(SubnetIDVisitor)
}

fn eth_addr_str_to_address(s: &str) -> anyhow::Result<Address> {
    let addr = EthAddress::from_str(s)?;
    Ok(Address::from(addr))
}

/// A serde deserialization method to deserialize cors origins from a sequence of strings,
/// e.g., [], ["*"], ["https://example.com", "https://www.example.org"].
pub fn deserialize_cors_origins<'de, D>(deserializer: D) -> anyhow::Result<AllowOrigin, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = AllowOrigin;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of strings")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut origins = Vec::new();
            while let Some(v) = seq.next_element::<String>()? {
                if v == "*" {
                    return Ok(AllowOrigin::any());
                } else {
                    origins.push(parse_origin(&v).map_err(Error::custom)?);
                }
            }
            Ok(AllowOrigin::list(origins))
        }
    }
    deserializer.deserialize_seq(Visitor)
}

fn parse_origin(s: &str) -> anyhow::Result<HeaderValue> {
    // First parse as url to extract the validated origin
    let origin = s.parse::<Url>()?.origin();
    if !origin.is_tuple() {
        return Err(anyhow!("opaque origins are not allowed"));
    }
    Ok(HeaderValue::from_str(&origin.ascii_serialization())?)
}

/// A serde deserialization method to deserialize cors methods from a sequence of strings,
/// e.g., [], ["*"], ["GET", "POST"].
pub fn deserialize_cors_methods<'de, D>(deserializer: D) -> anyhow::Result<AllowMethods, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = AllowMethods;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of strings")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut methods = Vec::new();
            while let Some(v) = seq.next_element::<String>()? {
                if v == "*" {
                    return Ok(AllowMethods::any());
                } else {
                    methods.push(v.parse().map_err(Error::custom)?);
                }
            }
            Ok(AllowMethods::list(methods))
        }
    }
    deserializer.deserialize_seq(Visitor)
}

/// A serde deserialization method to deserialize cors headers from a sequence of strings,
/// e.g., [], ["*"], ["Accept", "Content-Type"].
pub fn deserialize_cors_headers<'de, D>(deserializer: D) -> anyhow::Result<AllowHeaders, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = AllowHeaders;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of strings")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut headers = Vec::new();
            while let Some(v) = seq.next_element::<String>()? {
                if v == "*" {
                    return Ok(AllowHeaders::any());
                } else {
                    headers.push(v.parse().map_err(Error::custom)?);
                }
            }
            Ok(AllowHeaders::list(headers))
        }
    }
    deserializer.deserialize_seq(Visitor)
}
