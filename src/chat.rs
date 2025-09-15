use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    auth::Claims,
    db::{MessageInput, MsgListner},
    AppState,
};

use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};

use crate::db::{postgres::PostgresDb, Db};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    claims: Claims,
    State(state): State<AppState<PostgresDb>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state.db, claims.sub))
}

async fn handle_socket(socket: WebSocket, db: PostgresDb, user_id: usize) {
    let (sender, receiver) = socket.split();
    let state_clone = db.clone();

    tokio::spawn(write(sender, state_clone, user_id));
    tokio::spawn(read(receiver, db));
}

async fn read(mut receiver: SplitStream<WebSocket>, db: PostgresDb) {
    while let Some(result) = receiver.next().await {
        match result {
            Ok(Message::Text(text)) => match serde_json::from_str::<MessageInput>(&text) {
                Ok(msg) => {
                    if let Err(e) = db.create_message(msg).await {
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

async fn write(mut sender: SplitSink<WebSocket, Message>, db: PostgresDb, id: usize) {
    let messaged_users = db.get_messaged_users(id).await.unwrap();
    for user in messaged_users {
        let messages = db
            .get_messages_between_users(id, user.user_id)
            .await
            .unwrap();
        for message in messages {
            if sender
                .send(serde_json::to_string(&message).expect("").into())
                .await
                .is_err()
            {
                // disconect
                return;
            }
        }
    }
    let mut listener = db.listen_for_messages(id).await.unwrap();
    loop {
        let message = listener.receive().await.unwrap();
        if sender
            .send(serde_json::to_string(&message).expect("").into())
            .await
            .is_err()
        {
            // disconect
            return;
        }
    }
}
