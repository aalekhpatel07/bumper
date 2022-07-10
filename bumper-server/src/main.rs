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

use uuid::{Uuid, uuid};
use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use bumper_core::Car;

// use bumper_core::models::{web, car};

use futures::{TryStreamExt, StreamExt, future, pin_mut};
use futures_channel::mpsc::{unbounded, UnboundedSender};

use serde::{Serialize, Deserialize};
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type PeerUuidMap = Arc<Mutex<HashMap<SocketAddr, Uuid>>>;
type UuidCarMap = Arc<Mutex<HashMap<Uuid, Car>>>;

async fn handle_connection(
    peer_map: PeerMap,
    peer_uuid_map: PeerUuidMap,
    uuid_car_map: UuidCarMap,
    raw_stream: TcpStream,
    addr: SocketAddr
) {
    println!("Incoming TCP connection from: {}", addr);

    // let example_str = serde_json::to_string(&example).unwrap();

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let car = Car::new(100., 100., 60., 80.);
    // let car_web = web::Car::new(100., 100., 60., 80.);

    let car_to_send = serde_json::to_string(&car).unwrap();

    let car_uuid = Uuid::parse_str(&car.id).unwrap();
    peer_uuid_map.lock().unwrap().insert(addr, car_uuid);
    uuid_car_map.lock().unwrap().insert(car_uuid, car.clone());

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    tx.unbounded_send(Message::Text(car_to_send)).unwrap();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());

        let peers = peer_map.lock().unwrap();

        // We want to broadcast the message to everyone except ourselves.
        let broadcast_recipients =
            peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

        let all_cars = vec![
            car.clone(),
            car.clone()
        ];

        let cars_to_send = serde_json::to_string(&all_cars).unwrap();
        // We just learnt of the movement of the peer's car.

        for recp in broadcast_recipients {
            recp.unbounded_send(Message::Text(cars_to_send.clone())).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);

    peer_map.lock().unwrap().remove(&addr);
    if let Some(car_uuid) = peer_uuid_map.lock().unwrap().get(&addr) {
        uuid_car_map.lock().unwrap().remove(car_uuid);
    }
    peer_uuid_map.lock().unwrap().remove(&addr);

}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let peer_map = PeerMap::new(Mutex::new(HashMap::new()));
    let peer_uuid_map = PeerUuidMap::new(Mutex::new(HashMap::new()));
    let uuid_car_map = UuidCarMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(
            handle_connection(
                peer_map.clone(),
                peer_uuid_map.clone(),
                uuid_car_map.clone(),
                stream,
                addr
            )
        );
    }

    Ok(())
}