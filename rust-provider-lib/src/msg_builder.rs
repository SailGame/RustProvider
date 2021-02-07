use crate::core_pb::{
    ProviderMsg, CloseGameArgs, ErrorNumber, NotifyMsgArgs, provider_msg::Msg,
    StartGameArgs
};
use prost::Message;
use crate::util;

pub fn notify_msg_args<T: Message>(
    sequence_id: i32, err: ErrorNumber, room_id: i32, user_id: i32, custom: T, type_url: String
) -> ProviderMsg {
    // let type_url = String::from("type.googleapis.com/Uno.NotifyMsg");
    ProviderMsg {
        sequence_id,
        msg: Some(Msg::NotifyMsgArgs(NotifyMsgArgs {
            err: err as i32,
            room_id,
            user_id,
            custom: Some(util::pack(custom, type_url))
        }))
    }
}

pub fn close_game_args(
    sequence_id: i32, room_id: i32
) -> ProviderMsg {
    ProviderMsg {
        sequence_id,
        msg: Some(Msg::CloseGameArgs(CloseGameArgs {
            room_id
        }))
    }
}

// for tests
pub fn start_game_args<T: Message>(
    room_id: i32, user_id: Vec<u32>, custom: T, type_url: String
) -> StartGameArgs {
    StartGameArgs {
        room_id, 
        user_id,
        custom: Some(util::pack(custom, type_url))
    }
}