use rust_provider_lib::core_pb::{ErrorNumber, ProviderMsg, StartGameArgs};
use rust_provider_lib::msg_builder as pmb;
use crate::splendor_pb::{
    GameStart, NotifyMsg, notify_msg, GameState, UserOperation, ReserveFromDeckRsp,
    Development, Take, Purchase, Reserve, StartGameSettings
};
use prost::Message;

pub fn game_start(game_start: GameStart) -> NotifyMsg {
    NotifyMsg {
        msg: Some(notify_msg::Msg::GameStart(game_start))
    }
}

pub fn game_state(game_state: GameState) -> NotifyMsg {
    NotifyMsg {
        msg: Some(notify_msg::Msg::State(game_state))
    }
}

pub fn user_op(user_op: UserOperation) -> NotifyMsg {
    NotifyMsg {
        msg: Some(notify_msg::Msg::LastUserOperation(user_op))
    }
}

pub fn reserve_rsp(dev_card: Development) -> NotifyMsg {
    NotifyMsg {
        msg: Some(notify_msg::Msg::ReserveRsp(ReserveFromDeckRsp {
            card: Some(dev_card)
        }))
    }
}

pub fn notify_msg_args<T: Message>(
    room_id: i32, user_id: i32, custom: T
) -> ProviderMsg {
    pmb::notify_msg_args(
        0, ErrorNumber::Ok, room_id, user_id, custom, 
        String::from("type.googleapis.com/Splendor.NotifyMsg"))
}

// for tests
pub fn start_game_args<T: Message>(
    room_id: i32, user_id: Vec<u32>, custom: T
) -> StartGameArgs {
    pmb::start_game_args(room_id, user_id, custom, 
        String::from("type.googleapis.com/Splendor.StartGameSettings"))
}

pub fn take(resources: Vec<i32>) -> Take {
    Take { resources }
}

pub fn purchase(development_level: i32, index: i32) -> Purchase {
    Purchase { development_level, index }
}

pub fn reserve(development_level: i32, index: i32) -> Reserve {
    Reserve { development_level, index }
}

pub fn game_settings(round_time: i32) -> StartGameSettings {
    StartGameSettings { round_time }
}