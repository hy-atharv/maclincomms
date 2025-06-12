use std::{sync::{mpsc::{self}, Arc}};

use futures_util::{SinkExt, stream::{SplitSink}};
use tokio::{io::{AsyncRead, AsyncWrite}, sync::{Mutex}};
use tokio_tungstenite::{tungstenite::{self, Message}, WebSocketStream};

use crate::user_model::{DisconnectType, MessageType, RoomMessageType, RoomSenderMessage, SocketMessage, WhisperMode};




pub async fn send_messages(
    arc_write: Arc<Mutex<SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin + Send + 'static>, Message>>>, 
    user: String, 
    outgoing_rx: mpsc::Receiver<SocketMessage>
) {

    while let msg_res = outgoing_rx.recv() {  

        match msg_res {
            Ok(socket_message) => {
                let mut write = arc_write.lock().await;
                match socket_message {
                    SocketMessage::Message(MessageType::WORLD_CHAT(msg)) => {
                        let json_msg = serde_json::to_string(&msg).unwrap(); // Serialize to JSON
                        if let Err(e) = write.send(Message::Text(json_msg)).await {
                            break;
                        }
                    }
                    SocketMessage::Join(MessageType::WORLD_CHAT(msg)) => {
                        let json_msg = serde_json::to_string(&msg).unwrap(); // Serialize to JSON
                        if let Err(e) = write.send(Message::Text(json_msg)).await {
                            break;
                        }
                    }
                    SocketMessage::Leave(MessageType::WORLD_CHAT(msg)) => {
                        let json_msg = serde_json::to_string(&msg).unwrap(); // Serialize to JSON
                        if let Err(e) = write.send(Message::Text(json_msg)).await {
                            break;
                        }
                    }
                    SocketMessage::Message(MessageType::ROOM(RoomMessageType::SENDER(msg))) => {
                        let json_msg = serde_json::to_string(&msg).unwrap(); // Serialize to JSON
                        match msg.whisper_mode{
                            WhisperMode::HIDE_FROM => {
                                let m = RoomSenderMessage{
                                    username: msg.username,
                                    content: "  ".to_string(), //Double Spaces
                                    users: msg.users,
                                    whisper_mode: WhisperMode::SHARE_WITH,
                                    is_join_leave_msg: true
                                };
                                let json = serde_json::to_string(&m).unwrap(); // Serialize to JSON
                                if let Err(e) = write.send(Message::Text(json)).await {
                                    break;
                                }
                            }
                            WhisperMode::SHARE_WITH => {
                                let m = RoomSenderMessage{
                                    username: msg.username,
                                    content: "  ".to_string(), //Double Spaces
                                    users: msg.users,
                                    whisper_mode: WhisperMode::HIDE_FROM,
                                    is_join_leave_msg: true
                                };
                                let json = serde_json::to_string(&m).unwrap(); // Serialize to JSON
                                if let Err(e) = write.send(Message::Text(json)).await {
                                    break;
                                }
                            }
                            WhisperMode::NONE => {
                                //No need to inform to change keys
                            }
                        }
                        if let Err(e) = write.send(Message::Text(json_msg)).await {
                            break;
                        }
                    }
                    SocketMessage::RoomSenderKey(sender_key) => {
                        if let Err(e) = write.send(Message::Binary(sender_key)).await {
                            break;
                        }
                    }
                    SocketMessage::Join(MessageType::ROOM(RoomMessageType::SENDER(msg))) => {
                        let json_msg = serde_json::to_string(&msg).unwrap(); // Serialize to JSON
                        if let Err(e) = write.send(Message::Text(json_msg)).await {
                            break;
                        }
                    }
                    SocketMessage::Leave(MessageType::ROOM(RoomMessageType::SENDER(msg))) => {
                        let json_msg = serde_json::to_string(&msg).unwrap(); // Serialize to JSON
                        if let Err(e) = write.send(Message::Text(json_msg)).await {
                            break;
                        }
                    }
                    SocketMessage::Message(MessageType::DM(msg)) => {
                        let json_msg = serde_json::to_string(&msg).unwrap(); // Serialize to JSON
                        if let Err(e) = write.send(Message::Text(json_msg)).await {
                            break;
                        }
                    }
                    SocketMessage::Acknowledgement(ack) => {
                        if let Err(e) = write.send(Message::Binary(ack)).await {
                            break;
                        }
                    }
                    SocketMessage::Join(MessageType::DM(msg)) => {
                        let json_msg = serde_json::to_string(&msg).unwrap(); // Serialize to JSON
                        if let Err(e) = write.send(Message::Text(json_msg)).await {
                            break;
                        }
                    }
                    SocketMessage::Leave(MessageType::DM(msg)) => {
                        let json_msg = serde_json::to_string(&msg).unwrap(); // Serialize to JSON
                        if let Err(e) = write.send(Message::Text(json_msg)).await {
                            break;
                        }
                    } 
                    SocketMessage::Disconnect(DisconnectType::WORLD_CHAT) => {
                        let close_msg = Message::Close(Some(tungstenite::protocol::CloseFrame {
                            code: tungstenite::protocol::frame::coding::CloseCode::Normal,
                            reason: "User requested disconnection".into(),
                        }));
                        if let Err(e) = write.send(close_msg).await {
                            break;
                        }
                    }
                    SocketMessage::Disconnect(DisconnectType::ROOM) => {
                        let close_msg = Message::Close(Some(tungstenite::protocol::CloseFrame {
                            code: tungstenite::protocol::frame::coding::CloseCode::Normal,
                            reason: "User requested disconnection".into(),
                        }));
                        if let Err(e) = write.send(close_msg).await {
                            break;
                        }
                    }
                    SocketMessage::Disconnect(DisconnectType::DM) => {
                        let close_msg = Message::Close(Some(tungstenite::protocol::CloseFrame {
                            code: tungstenite::protocol::frame::coding::CloseCode::Normal,
                            reason: "User requested disconnection".into(),
                        }));
                        if let Err(e) = write.send(close_msg).await {
                            break;
                        }
                    }
                    _=>{}
                }
            }
            Err(error) => {
                
            }
        }
    }
}



