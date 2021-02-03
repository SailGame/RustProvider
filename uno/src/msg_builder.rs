use crate::uno_pb::{
    Draw, Skip, Play, GameStart, NotifyMsg, notify_msg, DrawRsp, Card, 
};

pub fn game_start(
    init_handcards: Vec<Card>, flipped_card: Card, first_player: u32
) -> NotifyMsg {
    NotifyMsg {
        msg: Some(notify_msg::Msg::GameStart(GameStart {
            init_handcards: init_handcards,
            flipped_card: Some(flipped_card),
            first_player
        }))
    }
}

pub fn draw(number: i32) -> NotifyMsg {
    NotifyMsg {
        msg: Some(notify_msg::Msg::Draw(Draw {
            number
        }))
    }
}

pub fn draw_rsp(cards: Vec<Card>) -> NotifyMsg {
    NotifyMsg {
        msg: Some(notify_msg::Msg::DrawRsp(DrawRsp {
            cards
        }))
    }
}

pub fn skip() -> NotifyMsg {
    NotifyMsg {
        msg: Some(notify_msg::Msg::Skip(Skip {}))
    }
}
pub fn play(card: Card, next_color: i32) -> NotifyMsg {
    NotifyMsg {
        msg: Some(notify_msg::Msg::Play(Play {
            card: Some(card),
            next_color: next_color
        }))
    }
}