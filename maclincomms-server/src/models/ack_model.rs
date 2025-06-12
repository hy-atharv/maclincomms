//--------------MESSAGE ACKS (Acknowledgements)

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum AckType {
    ServerAck,
    ReceiverAck
}

impl AckType {
    pub fn byte(&self) -> Vec<u8> {
        match self {
            Self::ServerAck => vec![0x01],
            Self::ReceiverAck => vec![0x02],
        }
    }
}