use crate::splendor_pb::{
    GameStart, NotifyMsg, notify_msg, GameState, UserOperation, ReserveFromDeckRsp,
    Development
};

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
