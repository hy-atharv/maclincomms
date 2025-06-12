

pub struct Endpoints {
    pub register: &'static str,
    pub login: &'static str,
    pub authN: &'static str,
    pub new_token: &'static str,
    pub world_chat: &'static str,
    pub create_room: &'static str,
    pub join_room: &'static str,
    pub get_room_data: &'static str,
    pub room_chat: &'static str,
    pub add_user: &'static str,
    pub accept_user: &'static str,
    pub get_dms_data: &'static str,
    pub get_dm_chats_data: &'static str,
    pub upload_dm_chats_data: &'static str,
    pub dm_chat: &'static str,
    pub block_user: &'static str,
    pub unblock_user: &'static str,
    pub realtime_notifications: &'static str,
    pub queued_notifications: &'static str
}

impl Endpoints {
    pub fn new() -> Self {
        Self {
            register: Self::get_register_endpoint(),
            login: Self::get_login_endpoint(),
            authN: Self::get_authenticate_user_endpoint(),
            new_token: Self::get_new_token_endpoint(),
            world_chat: Self::get_world_chat_endpoint(),
            create_room: Self::get_create_room_endpoint(),
            join_room: Self::get_join_room_endpoint(),
            get_room_data: Self::get_room_data_endpoint(),
            room_chat: Self::get_room_chat_endpoint(),
            add_user: Self::get_add_user_endpoint(),
            accept_user: Self::get_accept_user_endpoint(),
            get_dms_data: Self::get_dms_data_endpoint(),
            get_dm_chats_data: Self::get_dm_chats_data_endpoint(),
            upload_dm_chats_data: Self::upload_dm_chats_data_endpoint(),
            dm_chat: Self::get_dm_chat_endpoint(),
            block_user: Self::get_block_user_endpoint(),
            unblock_user: Self::get_unblock_user_endpoint(),
            queued_notifications: Self::get_queued_notifications_endpoint(),
            realtime_notifications: Self::get_realtime_notifications_endpoint()
        }
    }

    pub fn get_register_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/register_user";
    }

    pub fn get_login_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/login_user";
    }

    pub fn get_authenticate_user_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/authN_user";
    }

    pub fn get_new_token_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/new_token";
    }

    pub fn get_world_chat_endpoint() -> &'static str {
        return "wss://maclincomms-server-v2-prvj.shuttle.app/world_chat";
    }

    pub fn get_create_room_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/create_room";
    }

    pub fn get_join_room_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/join_room";
    }

    pub fn get_room_data_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/room_data";
    }

    pub fn get_room_chat_endpoint() -> &'static str {
        return "wss://maclincomms-server-v2-prvj.shuttle.app/room_chat";
    }

    pub fn get_add_user_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/add_user";
    }

    pub fn get_accept_user_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/accept_user";
    }

    pub fn get_dms_data_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/get_dms";
    }

    pub fn get_dm_chats_data_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/get_dm_chats";
    }

    pub fn upload_dm_chats_data_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/upload_dm_chats";
    }

    pub fn get_dm_chat_endpoint() -> &'static str {
        return "wss://maclincomms-server-v2-prvj.shuttle.app/dm_chat";
    }

    pub fn get_block_user_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/block_user";
    }

    pub fn get_unblock_user_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/unblock_user";
    }

    pub fn get_realtime_notifications_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/realtime_notifications";
    }

    pub fn get_queued_notifications_endpoint() -> &'static str {
        return "https://maclincomms-server-v2-prvj.shuttle.app/queued_notifications";
    }
}