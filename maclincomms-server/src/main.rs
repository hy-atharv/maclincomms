use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::web::ServiceConfig;
use actix_web::{
    web, App, HttpServer
};
use actix_ws::Session;
use database::auth_db::keep_alive_supabase;
use database::redis_db::keep_alive_upstash;
use uuid::Uuid;

use shuttle_actix_web::ShuttleActixWeb;

use secret_store::set_secrets;

mod secret_store;
mod routes;
mod middleware;
mod models;
mod core;
mod database;


#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore
)
 -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    type WorldChatSharedState = Arc<Mutex<HashMap<Uuid, Session>>>;

    type RoomChatSharedState = Arc<Mutex<HashMap<(String, String), HashMap<String, Session>>>>;

    type DMChatSharedState = Arc<Mutex<HashMap<(String, String), HashMap<String, Session>>>>;

    let pub_chat_shared_state: WorldChatSharedState = Arc::new(Mutex::new(HashMap::new()));

    let rooms_shared_state: RoomChatSharedState = Arc::new(Mutex::new(HashMap::new()));

    let dms_shared_state: DMChatSharedState = Arc::new(Mutex::new(HashMap::new()));


    set_secrets(secrets); // Store secrets globally

    //Task to keep Databases alive during long inactivity periods
    tokio::spawn(async{
        println!("Running Periodic Keep Alive Thread");
        loop{
            if let Ok(subs) = keep_alive_upstash().await{
                println!("Kept Upstash Alive");
            }
            if let Ok(()) = keep_alive_supabase().await{
                println!("Kept Supabase Alive")
            }
            // Query again after 24 hours
            tokio::time::sleep(std::time::Duration::from_secs(60 * 60 * 24)).await;
        }
    });

    let config = move |cfg: &mut ServiceConfig| {
        cfg
        .app_data(web::Data::new(pub_chat_shared_state.clone()))
        .app_data(web::Data::new(rooms_shared_state.clone()))
        .app_data(web::Data::new(dms_shared_state.clone()))
            .service(routes::endpoints::register)
            .service(routes::endpoints::login)
            .service(routes::endpoints::authenticate_user)
            .service(routes::endpoints::request_new_token)
            .service(routes::endpoints::public_chat)
            .service(routes::endpoints::create_room)
            .service(routes::endpoints::join_room)
            .service(routes::endpoints::retrieve_room_data)
            .service(routes::endpoints::private_room_chat)
            .service(routes::endpoints::add_user)
            .service(routes::endpoints::accept_user)
            .service(routes::endpoints::get_dms_data)
            .service(routes::endpoints::get_dm_chats_data)
            .service(routes::endpoints::upload_dm_chats_data)
            .service(routes::endpoints::private_dm_chat)
            .service(routes::endpoints::block_user)
            .service(routes::endpoints::unblock_user)
            .service(routes::endpoints::realtime_notifications)
            .service(routes::endpoints::queued_notifications);
    };
   
    Ok(config.into())

}