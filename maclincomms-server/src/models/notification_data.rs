use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationData {
    pub n_type: NotificationType,
    pub from: String,
    pub to: String,
    pub content: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationType {
    MESSAGE,
    ADD_REQUEST,
    ACCEPTED
}