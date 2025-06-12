use chrono::Local;

mod tui_main;
mod register_user;
mod user_model;
mod login_user;
mod tui_widgets;
mod screens_model;
mod screen_inputs;
mod websockets;
mod crypto;
mod endpoints;
mod event_model;
mod network_jobs;
mod persistent_login;


pub fn get_current_time() -> String {
    let now = Local::now();
    now.format("%I:%M %p").to_string()
}

pub fn get_current_date() -> String {
    let now = Local::now();
    now.format("%d %B, %Y").to_string()
}

//Main Function
#[tokio::main]
async fn main() {

    tui_main::start_tui().await;

}