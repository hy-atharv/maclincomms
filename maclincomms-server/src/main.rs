use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::web::ServiceConfig;
use actix_web::{
    web, App, HttpServer
};
use actix_ws::Session;
use uuid::Uuid;

use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;

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
    type SharedState = Arc<Mutex<HashMap<Uuid, Session>>>;

    let shared_state: SharedState = Arc::new(Mutex::new(HashMap::new()));

    set_secrets(secrets); // Store secrets globally

    let config = move |cfg: &mut ServiceConfig| {
        cfg
        .app_data(web::Data::new(shared_state.clone()))
            .service(routes::endpoints::register)
            .service(routes::endpoints::login)
            .service(routes::endpoints::public_chat);
    };
   
    Ok(config.into())

}