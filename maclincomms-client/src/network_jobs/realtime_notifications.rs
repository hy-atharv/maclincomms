use std::sync::mpsc::Sender;

use reqwest::Client;
use tokio_util::io::StreamReader;
use tokio::io::{AsyncBufReadExt, BufReader};
use futures_util::StreamExt;

use crate::get_current_time;
use crate::event_model::Event::{self, IncomingRealtimeNotificationEvent};
use crate::user_model::NotificationData;
use super::queued_notifications::TempNotif;



pub async fn subscribe_to_realtime_notifications(
    token: String,
    notification_sender: Sender<Event>,
    realtime_notifications_endpoint: &str
) {
    
    let url = realtime_notifications_endpoint.to_string();

    let client = Client::new();

    let response = match client
        .get(url)
        .header("Authorization", token)
        .header("Accept", "text/event-stream")
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to connect to SSE stream: {}", e);
            return;
        }
    };

    let stream = response.bytes_stream();
    let reader = StreamReader::new(stream.map(|res| {
        res.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }));

    let mut lines = BufReader::new(reader).lines();

    while let Ok(Some(line)) = lines.next_line().await {
        if line.starts_with("data: ") {
            let json_str = line.trim_start_matches("data: ").trim();
            match serde_json::from_str::<TempNotif>(json_str) {
                Ok(temp) => {
                    let notif = NotificationData {
                        n_type: temp.n_type,
                        from: temp.from,
                        to: temp.to,
                        content: temp.content,
                        time: get_current_time(),
                    };

                    if let Err(e) = notification_sender.send(IncomingRealtimeNotificationEvent(notif)) {
                        eprintln!("Failed to send notification: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse SSE JSON: {}", e);
                }
            }
        }
    }
}
