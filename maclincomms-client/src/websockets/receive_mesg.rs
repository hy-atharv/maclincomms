
use std::{sync::mpsc};
use futures_util::{StreamExt, stream::SplitStream};
use tokio::{io::{AsyncRead, AsyncWrite}};
use tokio_tungstenite::{tungstenite::{Message}, WebSocketStream};

use crate::{event_model::Event, user_model::{AckType, DisconnectType, DmMessage, RoomReceiverMessage, SocketMessage, WorldChatMessage}};

pub async fn handle_incoming_public_messages(
    mut read: SplitStream<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>>,
    incoming_tx: mpsc::Sender<Event>,
    socket_closer_tx: mpsc::Sender<SocketMessage>
) {
    while let Some(msg) = read.next().await {
        
        match msg {
            Ok(Message::Text(text)) => {

                if let Ok(parsed) = serde_json::from_str::<WorldChatMessage>(&text) { // ✅ Deserialize JSON
                    
                    if let Err(e) = incoming_tx.send(
                        Event::IncomingPublicMessageEvent(
                            WorldChatMessage {
                                    username: parsed.username,
                                    content: parsed.content,
                                    is_join_leave_msg: parsed.is_join_leave_msg
                            }
                        )
                    ) {
                            eprintln!("Failed to send event: {}", e);
                    }
                   
                } else {
                    eprintln!("Received invalid message format");
                }
            }
            Ok(Message::Binary(bytes)) => {
                if bytes==AckType::ServerAck.byte(){
                    if let Err(e) = incoming_tx.send(
                        Event::IncomingPublicMessageAckEvent(AckType::ServerAck)
                    ) {
                        eprintln!("Failed to send event: {}", e);
                    }
                }
                //For Public Chat, for now no list of acknowledgements by people, and only by server
            }
            Ok(Message::Pong(bytes)) => {
                //Received pong by server when pinged
                //Connection stable and web socket alive
            }
            Ok(Message::Close(Some(reason))) => {
                if let Err(err) = socket_closer_tx.send(SocketMessage::Disconnect(DisconnectType::WORLD_CHAT)){
                    println!("Couldnt send disconnect event");
                }
                if let Err(e) = incoming_tx.send(
                    Event::ExitWorldChatEvent
                ){
                    eprintln!("Couldnt exit world chat screen");
                }
            }
            Err(ws_error) => eprintln!("Invalid WebSocket message: {}", ws_error),
            _ => {}
        }
    }
}


pub async fn handle_incoming_room_messages(
    mut read: SplitStream<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>>,
    incoming_tx: mpsc::Sender<Event>,
    socket_closer_tx: mpsc::Sender<SocketMessage>
) {
    while let Some(msg) = read.next().await {
        
        match msg {
            Ok(Message::Text(text)) => {

                if let Ok(parsed) = serde_json::from_str::<RoomReceiverMessage>(&text) { // ✅ Deserialize JSON
                    
                    if let Err(e) = incoming_tx.send(
                        Event::IncomingRoomMessageEvent(
                            RoomReceiverMessage {
                                    username: parsed.username,
                                    content: parsed.content,
                                    is_join_leave_msg: parsed.is_join_leave_msg
                            }
                        )
                    ) {
                        eprintln!("Failed to send event: {}", e);
                    }
                   
                } else {
                    eprintln!("Received invalid message format");
                }
            }
            Ok(Message::Binary(bytes)) => {
                if bytes==AckType::ServerAck.byte(){
                    if let Err(e) = incoming_tx.send(
                        Event::IncomingRoomMessageAckEvent(AckType::ServerAck)
                    ) {
                        eprintln!("Failed to send event: {}", e);
                    }
                }
                //For ROOMS, for now no list of acknowledgements by people, and only by server
                //Sender Key Descriptor
                else if bytes[0]==0x11{
                    if let Err(e) = incoming_tx.send(
                        Event::IncomingRoomSenderKeyMessageEvent(bytes)
                    ) {
                        eprintln!("Failed to send event: {}", e);
                    }
                }
                //Unknown Rotate Chain Key Informer message
                else if bytes[0]==0x33{
                    let u_bytes = &bytes[1..];
                    let u = match String::from_utf8(u_bytes.to_vec()){
                        Ok(name)=> name,
                        Err(err) => "".to_string()
                    };
                    if let Err(e) = incoming_tx.send(
                        Event::UnknownRotateRoomChainKeyEvent(u)
                    ) {
                        eprintln!("Failed to send event: {}", e);
                    }
                }
                
            }
            Ok(Message::Pong(bytes)) => {
                //Received pong by server when pinged
                //Connection stable and web socket alive
            }
            Ok(Message::Close(Some(reason))) => {
                if let Err(err) = socket_closer_tx.send(SocketMessage::Disconnect(DisconnectType::ROOM)){
                    println!("Couldnt send disconnect event");
                }
                if let Err(e) = incoming_tx.send(
                    Event::ExitRoomChatEvent
                ){
                    eprintln!("Couldnt exit room chat screen");
                }
            }
            Err(ws_error) => eprintln!("Invalid WebSocket message: {}", ws_error),
            _ => {}
        }
    }
}

pub async fn handle_incoming_dm_messages(
    mut read: SplitStream<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>>,
    incoming_tx: mpsc::Sender<Event>,
    socket_closer_tx: mpsc::Sender<SocketMessage>
) {
    while let Some(msg) = read.next().await {
        
        match msg {
            Ok(Message::Text(text)) => {

                if let Ok(parsed) = serde_json::from_str::<DmMessage>(&text) { // ✅ Deserialize JSON

                    if let Err(e) = incoming_tx.send(
                        Event::IncomingDMMessageEvent(
                            DmMessage {
                                    username: parsed.username,
                                    content: parsed.content,
                                    is_online_offline_msg: parsed.is_online_offline_msg
                            }
                        )
                    ) {
                        eprintln!("Failed to send event: {}", e);
                    }
                   
                } else {
                    eprintln!("Received invalid message format");
                }
            }
            Ok(Message::Binary(bytes)) => {
                if bytes==AckType::ServerAck.byte(){
                    if let Err(e) = incoming_tx.send(
                        Event::IncomingDMMessageAckEvent(AckType::ServerAck)
                    ) {
                        eprintln!("Failed to send event: {}", e);
                    }
                }
                else if bytes==AckType::ReceiverAck.byte(){
                    if let Err(e) = incoming_tx.send(
                        Event::IncomingDMMessageAckEvent(AckType::ReceiverAck)
                    ) {
                        eprintln!("Failed to send event: {}", e);
                    }
                }
            }
            Ok(Message::Pong(bytes)) => {
                //Received pong by server when pinged
                //Connection stable and web socket alive
            }
            Ok(Message::Close(Some(reason))) => {
                if let Err(err) = socket_closer_tx.send(SocketMessage::Disconnect(DisconnectType::DM)){
                    println!("Couldnt send disconnect event");
                }
                if let Err(e) = incoming_tx.send(
                    Event::ExitDmChatEvent
                ){
                    eprintln!("Couldnt exit room chat screen");
                }
            }
            Err(ws_error) => eprintln!("Invalid WebSocket message: {}", ws_error),
            _ => {}
        }
    }
}