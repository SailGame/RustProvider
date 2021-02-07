use rust_provider_lib::state_machine::StateMachine;
use rust_provider_lib::core_pb::{
    ProviderMsg, RegisterRet, StartGameArgs, QueryStateArgs, UserOperationArgs,
    ErrorNumber
};
use rust_provider_lib::util;
use crate::uno_pb::{Draw, Skip, Play, UserOperation, StartGameSettings,
    user_operation::Operation,
};
use crate::state::{GlobalState};
use crate::msg_builder as mb;
use rust_provider_lib::msg_builder as pmb;

pub struct UnoStateMachine {
    state: GlobalState,
}

impl StateMachine for UnoStateMachine {
    fn transition_register_ret(&mut self, msg: RegisterRet) -> Vec<ProviderMsg> {
        assert_eq!(msg.err, ErrorNumber::Ok as i32);
        vec![]
    }

    fn transition_start_game_args(&mut self, msg: StartGameArgs) -> Vec<ProviderMsg> {
        let game_settings = util::unpack::<StartGameSettings>(msg.custom.unwrap());
        let (userid_to_init_handcards, flipped_card, first_player) =
            self.state.new_game(msg.room_id, msg.user_id, game_settings);

        let mut msgs = vec![];
        for (userid, init_handcards) in userid_to_init_handcards.iter() {
            msgs.push(mb::notify_msg_args(msg.room_id, userid.clone() as i32, 
                mb::game_start(init_handcards.clone(), flipped_card.clone(), first_player)));
        }
        msgs
    }

    fn transition_query_state_args(&mut self, _msg: QueryStateArgs) -> Vec<ProviderMsg> {
        vec![]
    }

    fn transition_user_operation_args(&mut self, msg: UserOperationArgs) -> Vec<ProviderMsg> {
        self.state.cur_roomid = msg.room_id;
        self.state.cur_userid = msg.user_id as i32;
        self.transition_user_operation(util::unpack::<UserOperation>(msg.custom.unwrap()))
    }
}

impl UnoStateMachine {
    pub fn new() -> UnoStateMachine {
        UnoStateMachine{ state: GlobalState::new() }
    }

    fn transition_user_operation(&mut self, msg: UserOperation) -> Vec<ProviderMsg> {
        let msg = msg.operation.unwrap();
        match msg {
            Operation::Draw(msg) => self.transition_draw(msg),
            Operation::Skip(msg) => self.transition_skip(msg),
            Operation::Play(msg) => self.transition_play(msg),
            _ => vec![]
        }
    }

    fn transition_draw(&mut self, msg: Draw) -> Vec<ProviderMsg> {
        let roomid = self.state.cur_roomid;
        let userid = self.state.cur_userid as u32;
        let game_state = self.state.roomid_to_game_state.get_mut(&roomid).unwrap();
        let cards = game_state.deck.draw_num(msg.number);
        game_state.userid_to_player_state.get_mut(&userid).unwrap().handcards_num += msg.number;

        let mut msgs = vec![];
        msgs.push(mb::notify_msg_args(roomid, 0, mb::draw(msg.number)));
        msgs.push(mb::notify_msg_args(roomid, userid as i32, mb::draw_rsp(cards)));
        msgs
    }

    fn transition_skip(&mut self, _: Skip) -> Vec<ProviderMsg> {
        let roomid = self.state.cur_roomid;
        let _userid = self.state.cur_userid as u32;

        let mut msgs = vec![];
        msgs.push(mb::notify_msg_args(roomid, 0, mb::skip()));
        msgs
    }

    fn transition_play(&mut self, msg: Play) -> Vec<ProviderMsg> {
        let roomid = self.state.cur_roomid;
        let userid = self.state.cur_userid as u32;
        let game_state = self.state.roomid_to_game_state.get_mut(&roomid).unwrap();
        let card = msg.card.clone().unwrap();
        game_state.deck.discard(card);
        game_state.userid_to_player_state.get_mut(&userid).unwrap().handcards_num -= 1;

        let mut msgs = vec![];
        msgs.push(mb::notify_msg_args(roomid, 0, mb::play(msg.card.unwrap(), msg.next_color)));

        if game_state.userid_to_player_state.get_mut(&userid).unwrap().handcards_num == 0 {
            msgs.push(pmb::close_game_args(0, roomid));
            self.state.roomid_to_game_state.remove(&roomid);
        }

        msgs
    }
}