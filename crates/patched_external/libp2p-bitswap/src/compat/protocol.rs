// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//
// Forked from https://github.com/consensus-shipyard/libp2p-bitswap with assumed MIT license
// as per Cargo.toml: https://github.com/consensus-shipyard/libp2p-bitswap/blob/7dd9cececda3e4a8f6e14c200a4b457159d8db33/Cargo.toml#L7
//
// License headers added post-fork.
use crate::compat::{other, CompatMessage};
use futures::future::BoxFuture;
use futures::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use libp2p::core::{upgrade, InboundUpgrade, OutboundUpgrade, UpgradeInfo};
use std::{io, iter};

// Undocumented, but according to JS we our messages have a max size of 512*1024
// https://github.com/ipfs/js-ipfs-bitswap/blob/d8f80408aadab94c962f6b88f343eb9f39fa0fcc/src/decision-engine/index.js#L16
const MAX_BUF_SIZE: usize = 524_288;

#[derive(Clone, Debug, Default)]
pub struct CompatProtocol;

impl UpgradeInfo for CompatProtocol {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(b"/ipfs/bitswap/1.2.0")
    }
}

impl<TSocket> InboundUpgrade<TSocket> for CompatProtocol
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = InboundMessage;
    type Error = io::Error;
    type Future = BoxFuture<'static, Result<Self::Output, Self::Error>>;

    fn upgrade_inbound(self, mut socket: TSocket, _info: Self::Info) -> Self::Future {
        Box::pin(async move {
            let packet = upgrade::read_length_prefixed(&mut socket, MAX_BUF_SIZE)
                .await
                .map_err(other)?;
            socket.close().await?;
            let message = CompatMessage::from_bytes(&packet)?;
            Ok(InboundMessage(message))
        })
    }
}

impl UpgradeInfo for CompatMessage {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(b"/ipfs/bitswap/1.2.0")
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for CompatMessage
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = ();
    type Error = io::Error;
    type Future = BoxFuture<'static, Result<Self::Output, Self::Error>>;

    fn upgrade_outbound(self, mut socket: TSocket, _info: Self::Info) -> Self::Future {
        Box::pin(async move {
            let bytes = self.to_bytes()?;
            upgrade::write_length_prefixed(&mut socket, bytes).await?;
            socket.close().await?;
            Ok(())
        })
    }
}

#[derive(Debug)]
pub struct InboundMessage(pub Vec<CompatMessage>);

impl From<()> for InboundMessage {
    fn from(_: ()) -> Self {
        Self(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{BitswapRequest, RequestType};
    use async_std::net::{TcpListener, TcpStream};
    use futures::prelude::*;
    use libipld::Cid;
    use libp2p::core::upgrade;

    #[async_std::test]
    async fn test_upgrade() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let listener_addr = listener.local_addr().unwrap();

        let server = async move {
            let incoming = listener.incoming().into_future().await.0.unwrap().unwrap();
            upgrade::apply_inbound(incoming, CompatProtocol)
                .await
                .unwrap();
        };

        let client = async move {
            let stream = TcpStream::connect(&listener_addr).await.unwrap();
            upgrade::apply_outbound(
                stream,
                CompatMessage::Request(BitswapRequest {
                    ty: RequestType::Have,
                    cid: Cid::default(),
                }),
                upgrade::Version::V1,
            )
            .await
            .unwrap();
        };

        future::select(Box::pin(server), Box::pin(client)).await;
    }
}
