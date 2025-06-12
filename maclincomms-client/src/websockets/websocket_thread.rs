use std::{sync::{mpsc, Arc}, time::Duration};     

use futures_util::{SinkExt, StreamExt};
use tokio::{ sync::{ Mutex}, task};
use tokio_tungstenite::{connect_async, tungstenite::{ClientRequestBuilder, Error, Message}};

use crate::{event_model::Event, tui_main::MaclincommsApp, user_model::SocketMessage};

use super::{receive_mesg, send_mesg};




pub async fn start_worldchat_websocket_task(
    app: &mut MaclincommsApp,
    username: String, 
    token: String, 
    endpoint: &'static str, 
    //chat_history: Arc<Mutex<Vec<(String, Line<'static>, String)>>>,
    incoming_tx: mpsc::Sender<Event>
) {
    
    //OTHER CHANNEL FOR SENDING AND RECEIVING OUTGOING MESGS BETWEEN USER UI AND WS IN PUB CHAT
    let (outgoing_tx, outgoing_rx) = mpsc::channel::<SocketMessage>();

    let socket_closer_tx = outgoing_tx.clone();

    app.outgoing_worldchat_msg_tx = Some(outgoing_tx);

    tokio::spawn(async move{
            
            let url = endpoint.parse().unwrap();


            let mut request = ClientRequestBuilder::new(url).with_header("Authorization", token);

            let ws_stream = match connect_async(request).await {
                Ok((ws_stream, _)) => {
                    ws_stream
                }
                Err(err) => {
                    return; // Exit early if connection fails
                }
            };

            let (mut write, mut read) = ws_stream.split();

            let arc_write = Arc::new(Mutex::new(write));

            // Keep Alive Web Socket Connection by pinging every 55 seconds
            let write_clone = arc_write.clone();
            task::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(55)).await;
                    // Lock the writer fresh each time
                    let mut keep_alive_writer = write_clone.lock().await;
                    if let Err(e) = keep_alive_writer.send(Message::Ping(vec![0x00])).await {
                        eprintln!("Ping failed: {}", e);
                    }
                }
            });


            // Handle incoming messages in a separate task
            let read_task = tokio::spawn(receive_mesg::handle_incoming_public_messages(read, incoming_tx, socket_closer_tx));


            // Receive from Ui channel and send messages
            let write_task = tokio::spawn(send_mesg::send_messages(arc_write, username, outgoing_rx));

            let _ = tokio::try_join!(read_task, write_task);

        });
}





pub async fn start_roomchat_websocket_task(
    app: &mut MaclincommsApp,
    username: String, 
    token: String, 
    endpoint: &'static str, 
    //chat_history: Arc<Mutex<Vec<(String, Line<'static>, String)>>>,
    incoming_tx: mpsc::Sender<Event>
) {
    
    //OTHER CHANNEL FOR SENDING AND RECEIVING OUTGOING MESGS BETWEEN USER UI AND WS IN ROOM CHAT
    let (outgoing_tx, outgoing_rx) = mpsc::channel::<SocketMessage>();

    let socket_closer_tx = outgoing_tx.clone();

    app.outgoing_roomchat_msg_tx = Some(outgoing_tx);

    tokio::spawn(async move{
        
        let url = endpoint.parse().unwrap();


        let mut request = ClientRequestBuilder::new(url).with_header("Authorization", token);

        let ws_stream = match connect_async(request).await {
            Ok((ws_stream, _)) => {
                ws_stream
            }
            Err(err) => {
                return; // Exit early if connection fails
            }
        };

        let (mut write, mut read) = ws_stream.split();

        let arc_write = Arc::new(Mutex::new(write));

        // Keep Alive Web Socket Connection by pinging every 55 seconds
        let write_clone = arc_write.clone();
        task::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(55)).await;
                // Lock the writer fresh each time
                let mut keep_alive_writer = write_clone.lock().await;
                if let Err(e) = keep_alive_writer.send(Message::Ping(vec![0x00])).await {
                    eprintln!("Ping failed: {}", e);
                }
            }
        });


        // Handle incoming messages in a separate task
        let read_task = tokio::spawn(receive_mesg::handle_incoming_room_messages(read, incoming_tx, socket_closer_tx));


        // Receive from Ui channel and send messages
        let write_task = tokio::spawn(send_mesg::send_messages(arc_write, username, outgoing_rx));

        let _ = tokio::try_join!(read_task, write_task);

    });
}



pub async fn start_dmchat_websocket_task(
    app: &mut MaclincommsApp,
    username: String, 
    target_username: String,
    token: String, 
    endpoint: &'static str, 
    //chat_history: Arc<Mutex<Vec<(String, Line<'static>, String)>>>,
    incoming_tx: mpsc::Sender<Event>
) -> Result<(), Error>{
    
    //OTHER CHANNEL FOR SENDING AND RECEIVING OUTGOING MESGS BETWEEN USER UI AND WS IN DM CHAT
    let (outgoing_tx, outgoing_rx) = mpsc::channel::<SocketMessage>();

    let socket_closer_tx = outgoing_tx.clone();

    let query = format!("?target={}",target_username);

    app.outgoing_dmchat_msg_tx = Some(outgoing_tx);


    let url = (endpoint.to_owned() + &query).parse().unwrap();


    let mut request = ClientRequestBuilder::new(url).with_header("Authorization", token);

    let ws_stream = match connect_async(request).await {
        Ok((ws_stream, _)) => {
            ws_stream
        }
        Err(err) => {
            
            return Err(err); // Exit early if connection fails
        }
    };

    tokio::spawn(async move{

        let (mut write, mut read) = ws_stream.split();

        let arc_write = Arc::new(Mutex::new(write));

        // Keep Alive Web Socket Connection by pinging every 55 seconds
        let write_clone = arc_write.clone();
        task::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(55)).await;
                // Lock the writer fresh each time
                let mut keep_alive_writer = write_clone.lock().await;
                if let Err(e) = keep_alive_writer.send(Message::Ping(vec![0x00])).await {
                    eprintln!("Ping failed: {}", e);
                }
            }
        });

        // Handle incoming messages in a separate task
        let read_task = tokio::spawn(receive_mesg::handle_incoming_dm_messages(read, incoming_tx, socket_closer_tx));


        // Receive from Ui channel and send messages
        let write_task = tokio::spawn(send_mesg::send_messages(arc_write, username, outgoing_rx));

        let _ = tokio::try_join!(read_task, write_task);

    });

    return Ok(());
}













// pub fn start_websocket_thread(
//     username: String, 
//     token: String, 
//     endpoint: &'static str, 
//     chat_history: Arc<Mutex<Vec<(String, Line<'static>, String)>>>,
//     rx: UnboundedReceiver<UserMessage>
// ) {

//     thread::spawn(move || {
//         let rt = Runtime::new().unwrap();
        
//         rt.block_on(async {
            
//             let url = endpoint.parse().unwrap();


//             let mut request = ClientRequestBuilder::new(url).with_header("Authorization", token);

//             let ws_stream = match connect_async(request).await {
//                 Ok((ws_stream, _)) => {
//                     ws_stream
//                 }
//                 Err(err) => {
//                     return; // Exit early if connection fails
//                 }
//             };

//             let (mut write, mut read) = ws_stream.split();


//             // Handle incoming messages in a separate task
//             let read_task = tokio::spawn(receive_mesg::handle_incoming_messages(read, chat_history));

//             // Read from command line and send messages
//             let write_task = tokio::spawn(send_mesg::send_messages(write, username, rx));

//             let _ = tokio::try_join!(read_task, write_task);

//         });
//     });
// }