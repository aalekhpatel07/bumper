//! A chat server that broadcasts a message to all connections.
//!
//! This is a simple line-based server which accepts WebSocket connections,
//! reads lines from those connections, and broadcasts the lines to all other
//! connected clients.
//!
//! You can test this out by running:
//!
//!     cargo run --example server 127.0.0.1:12345
//!
//! And then in another window run:
//!
//!     cargo run --example client ws://127.0.0.1:12345/
//!
//! You can run the second command in multiple windows and then chat between the
//! two, seeing the messages from the other client as they're received. For all
//! connected clients they'll all join the same room and see everyone else's
//! messages.

use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use bumper_server::{BumperCars, Game};

use log::{debug, error, info, warn};
use simple_logger::SimpleLogger;

// use bumper_core::models::{web, car};

use futures::{future, pin_mut, StreamExt, TryStreamExt};
use futures_channel::mpsc::{unbounded, UnboundedSender};

use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
// type PeerCarMap = Arc<Mutex<HashMap<SocketAddr, Car>>>;
// type UuidCarMap = Arc<Mutex<HashMap<Uuid, Car>>>;

pub fn set_up_logging() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
}

async fn handle_connection(
    peer_map: PeerMap,
    game_state: BumperCars<SocketAddr>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    debug!("Incoming TCP connection from: {}", addr);

    // let example_str = serde_json::to_string(&example).unwrap();

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    debug!("WebSocket connection established: {}", addr);

    debug!("Creating player: {}", addr);
    game_state.create_player(addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();

    debug!("Sending player state to: {}", addr);
    tx.unbounded_send(Message::Text(
        game_state
            .send_player_state_to(addr)
            .expect("Couldn't create player state."),
    ))
    .expect("Couldn't send player state.");

    debug!("Game state: {:#?}", game_state.players);
    debug!("Sending game state to {}", addr);
    tx.unbounded_send(Message::Text(game_state.send_game_state_to(addr)))
        .expect("Couldn't send game state to player.");

    peer_map
        .lock()
        .expect("Failed to lock peer_map")
        .insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        debug!("Received a message from {}", addr);

        if let Ok(self_car_view) = serde_json::from_str(msg.to_text().unwrap()) {
            game_state.update_player(addr, self_car_view);
        } else {
            warn!("Couldn't parse message: {}", msg.to_text().unwrap());
        }

        let peers = peer_map.lock().unwrap();

        // We want to broadcast the message to everyone except ourselves.
        let broadcast_recipients = peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr);

        for (recp_addr, recp_socket) in broadcast_recipients {
            let to_send = game_state.send_game_state_to(*recp_addr);
            debug!("Sending {} to {}", to_send, recp_addr);
            if let Err(e) = recp_socket.unbounded_send(Message::Text(to_send)) {
                game_state.remove_player(*recp_addr);
                error!("Failed to send to {}: {}", recp_addr, e);
            }
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    {
        let peers = peer_map.lock().unwrap();

        // We want to broadcast the message to everyone except ourselves.
        let broadcast_recipients = peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr);

        for (recp_addr, recp_socket) in broadcast_recipients {
            let to_send = game_state.send_game_state_to(*recp_addr);
            debug!("Sending {} to {}", to_send, recp_addr);
            if let Err(e) = recp_socket.unbounded_send(Message::Text(to_send)) {
                game_state.remove_player(*recp_addr);
                error!("Failed to send to {}: {}", recp_addr, e);
            }
        }
    }

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    debug!("{} disconnected", &addr);
    debug!("Removing player: {}", addr);
    game_state.remove_player(addr);

    {
        let peers = peer_map.lock().unwrap();

        // We want to broadcast the message to everyone except ourselves.
        let broadcast_recipients = peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr);

        for (recp_addr, recp_socket) in broadcast_recipients {
            let to_send = game_state.send_game_state_to(*recp_addr);
            debug!("Sending {} to {}", to_send, recp_addr);
            if let Err(e) = recp_socket.unbounded_send(Message::Text(to_send)) {
                game_state.remove_player(*recp_addr);
                error!("Failed to send to {}: {}", recp_addr, e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    set_up_logging();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let peer_map = PeerMap::new(Mutex::new(HashMap::new()));
    // let peer_car_map = PeerCarMap::new(Mutex::new(HashMap::new()));
    let game_state = BumperCars::new();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            peer_map.clone(),
            game_state.clone(),
            stream,
            addr,
        ));
    }

    Ok(())
}
