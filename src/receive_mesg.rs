
use futures_util::{StreamExt, SinkExt, stream::{SplitSink, SplitStream}};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{self, AsyncBufReadExt};
use tokio_tungstenite::{connect_async, tungstenite::{client::{self, IntoClientRequest}, http::Request, ClientRequestBuilder, Message}, WebSocketStream};

pub async fn handle_incoming_messages(mut read: SplitStream<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>>) {
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                println!("{}", msg);
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
            }
        }
    }
}