use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};

use serde::{Deserialize, Serialize};
use serde_json;

use crate::db::{model::MessageInput, MsgListner};

use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};

use crate::db::{postgres::PostgresDb, Db};

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<PostgresDb>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: PostgresDb) {
    let id_msg = match socket.next().await {
        Some(Ok(Message::Text(text))) => text,
        Some(Ok(_)) => {
            eprintln!("Expected text message for ID");
            return;
        }
        Some(Err(e)) => {
            eprintln!("WebSocket error: {}", e);
            return;
        }
        None => {
            eprintln!("Connection closed before sending ID");
            return;
        }
    };

    // Parse the ID as usize
    let id: usize = match id_msg.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid ID received: {}", id_msg);
            return;
        }
    };

    let (sender, receiver) = socket.split();
    let state_clone = state.clone();

    tokio::spawn(write(sender, state_clone, id));
    tokio::spawn(read(receiver, state));
}

async fn read(mut receiver: SplitStream<WebSocket>, state: PostgresDb) {
    while let Some(result) = receiver.next().await {
        match result {
            Ok(Message::Text(text)) => match serde_json::from_str::<MessageStruct>(&text) {
                Ok(msg) => {
                    let input = MessageInput {
                        sender_id: msg.sender_id,
                        receiver_id: msg.reciver_id,
                        content: msg.message,
                    };

                    if let Err(e) = state.create_message(input).await {
                        eprintln!("Failed to insert message: {e}");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse message JSON: {e} messgae: {text}");
                }
            },
            Ok(Message::Binary(_)) => {}
            Ok(Message::Close(_)) => {
                println!("Client disconnected");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("WebSocket error: {e}");
                break;
            }
        }
    }
}

async fn write(mut sender: SplitSink<WebSocket, Message>, state: PostgresDb, id: usize) {
    let messaged_users = state.get_messaged_users(id).await.unwrap();
    // wyslij wszystkie wiadomosci do tego uzytkownika
    for user in messaged_users {
        let messages = state
            .get_messages_between_users(id, user.user_id)
            .await
            .unwrap();
        for message in messages {
            if sender
                .send(
                    serde_json::to_string(&MessageStruct {
                        sender_id: message.sender_id,
                        reciver_id: message.receiver_id,
                        message: message.content,
                        date: message.sent_at,
                    })
                    .expect("")
                    .into(),
                )
                .await
                .is_err()
            {
                // disconect
                return;
            }
        }
    }
    let listener = state.listen_for_messages(id).await.unwrap();
    loop {
        let message = listener.receive().await.unwrap();
        if sender
            .send(
                serde_json::to_string(&MessageStruct {
                    sender_id: message.sender_id,
                    reciver_id: message.receiver_id,
                    message: message.content,
                    date: message.sent_at,
                })
                .expect("")
                .into(),
            )
            .await
            .is_err()
        {
            // disconect
            return;
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MessageStruct {
    sender_id: usize,
    reciver_id: usize,
    message: String,
    date: u64,
}
