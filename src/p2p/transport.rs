use std::net::SocketAddr;

use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use futures_util::SinkExt;
use futures_util::StreamExt;

use tracing::{debug, info, warn};

use crate::consensus::ConsensusState;
use super::message::P2PMessage;

/// Accepts inbound TCP connections on the specified `addr`.
/// Each connection is handled in a new task by `handle_connection`.
///
/// # Arguments
///
/// * `cs` - The shared consensus state (used to process inbound messages).
/// * `addr` - The listening address (host:port).
pub async fn accept_loop(cs: ConsensusState, addr: SocketAddr) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {} ...", addr);

    loop {
        // Accept a new socket
        let (socket, _remote_addr) = listener.accept().await?;
        let cs_clone = cs.clone();

        // Spawn a task to handle the new connection
        tokio::spawn(async move {
            if let Err(e) = handle_connection(cs_clone, socket).await {
                warn!("Inbound connection error: {:?}", e);
            }
        });
    }
}

/// Attempts to connect to a peer at `addr`.
/// Upon success, spawns a new task to handle the connection.
///
/// # Arguments
///
/// * `cs` - The shared consensus state.
/// * `addr` - The remote peer's address.
pub async fn connect_to_peer(cs: ConsensusState, addr: SocketAddr) -> Result<()> {
    debug!("Connecting to {}", addr);
    let socket = TcpStream::connect(addr).await?;
    let cs_clone = cs.clone();

    tokio::spawn(async move {
        if let Err(e) = handle_connection(cs_clone, socket).await {
            warn!("Outbound connection error: {:?}", e);
        }
    });

    Ok(())
}

/// Handles a single inbound or outbound TCP connection.
///
/// Uses a length-delimited codec to separate messages. Each message is
/// expected to be valid JSON (deserialized into `P2PMessage`). If successful,
/// the message is passed to `cs.process_p2p_message`.
///
/// # Arguments
///
/// * `cs` - The shared consensus state.
/// * `socket` - The TCP stream to handle.
async fn handle_connection(cs: ConsensusState, socket: TcpStream) -> Result<()> {
    let mut framed = Framed::new(socket, LengthDelimitedCodec::new());

    while let Some(Ok(bytes)) = framed.next().await {
        let msg_json = String::from_utf8(bytes.to_vec())?;
        let msg: P2PMessage = serde_json::from_str(&msg_json)?;

        // Process the inbound message
        cs.process_p2p_message(msg).await?;
    }

    Ok(())
}

/// Sends a single message (`msg`) to a peer at `addr`.
///
/// **Note**: This example opens a *new* TCP connection each time,
/// which is inefficient. A production system would typically maintain
/// a persistent connection and reuse it.
///
/// # Arguments
///
/// * `addr` - The peer's address to connect.
/// * `msg` - The message to send.
pub async fn send_message(addr: SocketAddr, msg: &P2PMessage) -> Result<()> {
    let socket = TcpStream::connect(addr).await?;
    let mut framed = Framed::new(socket, LengthDelimitedCodec::new());
    let msg_json = serde_json::to_vec(msg)?;
    framed.send(msg_json.into()).await?;
    Ok(())
}

