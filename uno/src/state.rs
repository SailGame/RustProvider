use std::collections::HashMap;
use crate::uno_pb::{StartGameSettings, Card, CardColor, CardText};
use crate::card::{Deck};
use rand::Rng;

pub struct GlobalState {
    pub roomid_to_game_state: HashMap<i32, GameState>,
    pub cur_roomid: i32,
    pub cur_userid: i32,
}

impl GlobalState {
    pub fn new() -> GlobalState {
        GlobalState {
            roomid_to_game_state: HashMap::new(),
            cur_roomid: -1,
            cur_userid: -1
        }
    }

    pub fn new_game(
        &mut self, roomid: i32, userids: Vec<u32>, game_settings: StartGameSettings
    ) -> (HashMap<u32, Vec<Card>>, Card, u32) {
        assert!(!self.roomid_to_game_state.contains_key(&roomid));
        self.roomid_to_game_state.insert(roomid, GameState::new(userids, game_settings));
        self.roomid_to_game_state.get_mut(&roomid).unwrap().game_start()
    }
}

pub struct GameState {
    pub player_num: usize,
    pub game_settings: StartGameSettings,
    pub deck: Deck,
    pub userid_to_player_state: HashMap<u32, PlayerState>,
}

impl GameState {
    pub fn new(
        userids: Vec<u32>, game_settings: StartGameSettings
    ) -> GameState {
        let mut userid_to_player_state = HashMap::new();
        for userid in userids.iter() {
            userid_to_player_state.insert(userid.clone(), PlayerState::new());
        }

        GameState {
            player_num: userids.len(),
            game_settings,
            deck: Deck::new(),
            userid_to_player_state
        }
    }

    pub fn game_start(&mut self) -> (HashMap<u32, Vec<Card>>, Card, u32) {
        let init_handcards = self.deck.deal_init_handcards(self.player_num);
        let mut userid_to_init_handcards = HashMap::new();

        for (i, (userid, _)) in self.userid_to_player_state.iter().enumerate() {
            userid_to_init_handcards.insert(userid.clone(), init_handcards[i].clone());
        }

        let mut flipped_card;
        let draw_texts = [CardText::DrawTwo as i32, CardText::DrawFour as i32];
        loop {
            flipped_card = self.deck.draw();
            if flipped_card.color == CardColor::Black as i32 {
                self.deck.put_to_bottom(flipped_card);
            }
            else {
                if draw_texts.contains(&flipped_card.text) {
                    flipped_card.text = CardText::Empty as i32;
                }
                break;
            }
        }

        let first_player = rand::thread_rng().gen_range(0..self.player_num);

        (userid_to_init_handcards, flipped_card, first_player as u32)
    }
}

pub struct PlayerState {
    pub handcards_num: i32,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState{ handcards_num: 7 }
    }
}