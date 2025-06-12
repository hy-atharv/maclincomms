use crate::user_model::{AckType, ChatData, DmMessage, NotificationData, RoomReceiverMessage, WorldChatMessage};



pub enum Event {
    IncomingPublicMessageEvent(WorldChatMessage),
    IncomingPublicMessageAckEvent(AckType),
    IncomingRoomMessageEvent(RoomReceiverMessage),
    IncomingRoomMessageAckEvent(AckType),
    IncomingRoomSenderKeyMessageEvent(Vec<u8>),
    UnknownRotateRoomChainKeyEvent(String),
    LoadDmsDataEvent,
    IncomingDMMessageEvent(DmMessage),
    IncomingDMMessageAckEvent(AckType),
    IncomingRealtimeNotificationEvent(NotificationData),
    InputEvent(crossterm::event::KeyEvent),
    RegisterEvent,
    LoginEvent,
    RoomCreationEvent,
    RoomJoinEvent,
    RoomChatEvent(String), //With room token,
    DmChatEvent(String), //With dm token
    UploadDmChatsEvent(ChatData),
    ExitWorldChatEvent,
    ExitRoomChatEvent,
    ExitDmChatEvent,
    BlockEvent,
    UnblockEvent,
    AddUserEvent,
    AcceptUserEvent(String) //With username to accept
}