// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//
// Forked from https://github.com/consensus-shipyard/libp2p-bitswap with assumed MIT license
// as per Cargo.toml: https://github.com/consensus-shipyard/libp2p-bitswap/blob/7dd9cececda3e4a8f6e14c200a4b457159d8db33/Cargo.toml#L7
//
// License headers added post-fork.
use crate::compat::other;
use crate::compat::prefix::Prefix;
use crate::protocol::{BitswapRequest, BitswapResponse, RequestType};
use libipld::Cid;
use prost::Message;
use std::convert::TryFrom;
use std::io;

mod bitswap_pb {
    include!(concat!(env!("OUT_DIR"), "/bitswap_pb.rs"));
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompatMessage {
    Request(BitswapRequest),
    Response(Cid, BitswapResponse),
}

impl CompatMessage {
    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut msg = bitswap_pb::Message::default();
        match self {
            CompatMessage::Request(BitswapRequest { ty, cid }) => {
                let mut wantlist = bitswap_pb::message::Wantlist::default();
                let entry = bitswap_pb::message::wantlist::Entry {
                    block: cid.to_bytes(),
                    want_type: match ty {
                        RequestType::Have => bitswap_pb::message::wantlist::WantType::Have,
                        RequestType::Block => bitswap_pb::message::wantlist::WantType::Block,
                    } as _,
                    send_dont_have: true,
                    cancel: false,
                    priority: 1,
                };
                wantlist.entries.push(entry);
                msg.wantlist = Some(wantlist);
            }
            CompatMessage::Response(cid, BitswapResponse::Have(have)) => {
                let block_presence = bitswap_pb::message::BlockPresence {
                    cid: cid.to_bytes(),
                    r#type: if *have {
                        bitswap_pb::message::BlockPresenceType::Have
                    } else {
                        bitswap_pb::message::BlockPresenceType::DontHave
                    } as _,
                };
                msg.block_presences.push(block_presence);
            }
            CompatMessage::Response(cid, BitswapResponse::Block(bytes)) => {
                let payload = bitswap_pb::message::Block {
                    prefix: Prefix::from(cid).to_bytes(),
                    data: bytes.to_vec(),
                };
                msg.payload.push(payload);
            }
        }
        let mut bytes = Vec::with_capacity(msg.encoded_len());
        msg.encode(&mut bytes).map_err(other)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> io::Result<Vec<Self>> {
        let msg = bitswap_pb::Message::decode(bytes)?;
        let mut parts = vec![];
        for entry in msg.wantlist.unwrap_or_default().entries {
            if !entry.send_dont_have {
                tracing::error!("message hasn't set `send_dont_have`: skipping");
                continue;
            }
            let cid = Cid::try_from(entry.block).map_err(other)?;
            let ty = match entry.want_type {
                ty if bitswap_pb::message::wantlist::WantType::Have as i32 == ty => {
                    RequestType::Have
                }
                ty if bitswap_pb::message::wantlist::WantType::Block as i32 == ty => {
                    RequestType::Block
                }
                _ => {
                    tracing::error!("invalid request type: skipping");
                    continue;
                }
            };
            parts.push(CompatMessage::Request(BitswapRequest { ty, cid }));
        }
        for payload in msg.payload {
            let prefix = Prefix::new(&payload.prefix)?;
            let cid = prefix.to_cid(&payload.data)?;
            parts.push(CompatMessage::Response(
                cid,
                BitswapResponse::Block(payload.data.to_vec()),
            ));
        }
        for presence in msg.block_presences {
            let cid = Cid::try_from(presence.cid).map_err(other)?;
            let have = match presence.r#type {
                ty if bitswap_pb::message::BlockPresenceType::Have as i32 == ty => true,
                ty if bitswap_pb::message::BlockPresenceType::DontHave as i32 == ty => false,
                _ => {
                    tracing::error!("invalid block presence type: skipping");
                    continue;
                }
            };
            parts.push(CompatMessage::Response(cid, BitswapResponse::Have(have)));
        }
        Ok(parts)
    }
}
