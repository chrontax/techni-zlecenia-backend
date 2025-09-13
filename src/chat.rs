use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};

use serde::{Deserialize, Serialize};
use serde_json;

use std::process::Command;

use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let (sender, receiver) = socket.split();

    tokio::spawn(write(sender));
    tokio::spawn(read(receiver));
}

async fn read(mut receiver: SplitStream<WebSocket>) {
    while let Some(msg) = receiver.next().await {
        let msg = if let Ok(msg) = msg {
            dbg!(msg)
        } else {
            // client disconnected
            return;
        };
    }
}

async fn write(mut sender: SplitSink<WebSocket, Message>) {
    // wyslij wszystkie wiadomosci do tego uzytkownika
    let msg: MessageStruct = MessageStruct {
        sender_id: 1,
        reciver_id: 1,
        message: "lama ma kota".to_string(),
        date: 1,
    };
    for _ in 0..10 {
        if sender
            .send(serde_json::to_string(&msg).expect("").into())
            .await
            .is_err()
        {
            // disconect
            return;
        }
    }
    while true {
        let mut child = Command::new("sleep").arg("5").spawn().unwrap();
        let _result = child.wait().unwrap();
        if sender.send("lama".into()).await.is_err() {
            // disconect
            return;
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MessageStruct {
    sender_id: i16,
    reciver_id: i16,
    message: String,
    date: i64,
}
