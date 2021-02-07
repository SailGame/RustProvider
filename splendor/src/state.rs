use std::collections::HashMap;
use crate::splendor_pb;
use crate::splendor_pb::{
    StartGameSettings, Development, ResourceMap, ResourceEntry, ResourceType, 
    Noble, GameStart, DevelopmentLevelState
};
use rand::prelude::*;

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
    ) -> GameStart {
        assert!(!self.roomid_to_game_state.contains_key(&roomid));
        self.roomid_to_game_state.insert(roomid, GameState::new(userids, game_settings));
        self.roomid_to_game_state.get_mut(&roomid).unwrap().game_start()
    }
}

pub struct GameState {
    pub player_num: usize,
    pub game_settings: StartGameSettings,
    pub board_state: BoardState,
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
            board_state: BoardState::new(),
            userid_to_player_state
        }
    }

    pub fn game_start(&self) -> GameStart {
        GameStart {
            first_player: rand::thread_rng().gen_range(0..self.player_num) as i32,
            state: Some(self.get_pub_state())
        }
    }

    pub fn get_pub_state(&self) -> splendor_pb::GameState {
        let mut player_states = vec![];
        for state in self.userid_to_player_state.values() {
            player_states.push(state.get_pub_state());
        }
        splendor_pb::GameState {
            dev_level1: Some(self.board_state.dev_cards[0].get_pub_state()),
            dev_level2: Some(self.board_state.dev_cards[1].get_pub_state()),
            dev_level3: Some(self.board_state.dev_cards[2].get_pub_state()),
            resources_on_board: Some(self.board_state.resources_on_board.clone()),
            nobles_on_board: self.board_state.nobles_on_board.clone(),
            player_states
        }
    }
}

pub struct BoardState {
    pub dev_cards: [DevCardsLevelState; 3],
    pub resources_on_board: ResourceMap,
    pub nobles_on_board: Vec<Noble>
}

impl BoardState {
    pub fn new() -> BoardState {
        BoardState {
            dev_cards: [
                DevCardsLevelState::new_level1(),
                DevCardsLevelState::new_level2(),
                DevCardsLevelState::new_level3(),
            ],
            resources_on_board: ResourceMap {
                entries: vec![
                    ResourceEntry { resource_type: ResourceType::Agate as i32, number: 7 },
                    ResourceEntry { resource_type: ResourceType::Emerald as i32, number: 7 },
                    ResourceEntry { resource_type: ResourceType::Diamond as i32, number: 7 },
                    ResourceEntry { resource_type: ResourceType::Ruby as i32, number: 7 },
                    ResourceEntry { resource_type: ResourceType::Sapphire as i32, number: 7 },
                    ResourceEntry { resource_type: ResourceType::Gold as i32, number: 5 },
                ]
            },
            nobles_on_board: BoardState::rand_nobles()
        }
    }

    pub fn rand_nobles() -> Vec<Noble> {
        let mut nobles = vec![];
        for i in 0..5 {
            loop {
                // 3 nobles like 3, 3, 3 and 2 nobles like 4, 4
                let resource_type_num = if i < 3 { 3 } else { 2 };
                let numbers = if i < 3 { vec![3, 3, 3] } else { vec![4, 4] };

                let rands = get_multi_rand_nums(
                    resource_type_num, ResourceType::Agate as i32, ResourceType::Sapphire as i32);
                let noble = get_a_rand_noble(rands, numbers);
                if nobles.iter().all(|n| *n != noble) {
                    // no repatitive noble
                    nobles.push(noble);
                    break;
                }
            }
        }
        nobles
    }

    pub fn take_resource(&mut self, resource_type: i32) {
        let entry = &mut self.resources_on_board.entries[resource_type as usize];
        assert_eq!(entry.resource_type, resource_type);
        entry.number -= 1;
        assert!(entry.number >= 0);
    }

    pub fn return_resource(&mut self, res_map: ResourceMap) {
        for entry in res_map.entries.iter() {
            self.resources_on_board.entries[entry.resource_type as usize].number += entry.number;
        }
    }
}

pub struct DevCardsLevelState {
    pub deck: Vec<Development>,
    pub cards_on_board: Vec<Development>
}

impl DevCardsLevelState {
    pub fn new_level1() -> DevCardsLevelState {
        let templates = vec![
            vec![1, 1, 1, 1, 0], vec![1, 1, 2, 0], vec![2, 2, 0]
        ];
        let mut deck = get_rand_devs(templates, 40);
        let cards_on_board = deck.split_off(deck.len() - 4);
        DevCardsLevelState { deck, cards_on_board }
    }

    pub fn new_level2() -> DevCardsLevelState {
        let templates = vec![
            vec![2, 2, 2, 1], vec![1, 4, 1], vec![2, 3, 1], 
            vec![5, 2], vec![2, 4, 2], vec![3, 3, 1, 2], vec![2, 2, 2, 2, 2], vec![3, 2, 2, 2]
        ];
        let mut deck = get_rand_devs(templates, 30);
        let cards_on_board = deck.split_off(deck.len() - 4);
        DevCardsLevelState { deck, cards_on_board }
    }

    pub fn new_level3() -> DevCardsLevelState {
        let templates = vec![
            vec![6, 3], vec![2, 5, 3], vec![2, 3, 4, 3], vec![3, 3, 3, 3, 3],
            vec![7, 4], vec![2, 6, 4], vec![3, 3, 5, 4], vec![4, 4, 4, 4, 4],
            vec![8, 5], vec![2, 7, 5], vec![4, 4, 6, 5], vec![4, 5, 5, 5, 5]
        ];
        let mut deck = get_rand_devs(templates, 20);
        let cards_on_board = deck.split_off(deck.len() - 4);
        DevCardsLevelState { deck, cards_on_board }
    }

    pub fn get_pub_state(&self) -> DevelopmentLevelState {
        DevelopmentLevelState {
            deck_num: self.deck.len() as i32,
            cards: self.cards_on_board.clone()
        }
    }

    pub fn take(&mut self, index: i32) -> Development {
        if self.deck.is_empty() {
            self.cards_on_board.remove(index as usize)
        }
        else {
            let old = self.deck.pop().unwrap();
            self.cards_on_board[index as usize] = old.clone();
            old
        }
    }
}

pub struct PlayerState {
    pub dev_cards: Vec<Development>,
    pub resource: ResourceMap,
    pub nobles: Vec<Noble>,
    pub reserved_cards: Vec<Development>,
    pub points: i32
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState {
            dev_cards: vec![],
            resource: get_default_resource_map(),
            nobles: vec![],
            reserved_cards: vec![],
            points: 0
        }
    }

    pub fn get_pub_state(&self) -> splendor_pb::PlayerState {
        let mut res_map = get_default_resource_map();
        for card in self.dev_cards.iter() {
            res_map.entries[card.resource_type as usize].number += 1;
        }

        splendor_pb::PlayerState {
            development: Some(res_map),
            resource: Some(self.resource.clone()),
            nobles: self.nobles.clone(),
            reserved_num: self.reserved_cards.len() as i32,
            points: self.points
        }
    }

    pub fn take_resource(&mut self, resource_type: i32) {
        self.resource.entries[resource_type as usize].number += 1;
    }

    pub fn spend_resource(&mut self, res_map: ResourceMap) {
        for entry in res_map.entries.iter() {
            self.resource.entries[entry.resource_type as usize].number -= entry.number;
        }
    }
}

fn get_multi_rand_nums(num: usize, min: i32, max: i32) -> Vec<i32> {
    let mut nums: Vec<i32> = (min..max).collect();
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);
    nums.split_off(num)
}

fn get_a_rand_noble(rands: Vec<i32>, numbers: Vec<i32>) -> Noble {
    assert_eq!(rands.len(), numbers.len());
    let mut entries = vec![];
    for i in 0..numbers.len() {
        entries.push(ResourceEntry { resource_type: rands[i], number: numbers[i] });
    }
    Noble {
        capital: Some(ResourceMap { entries }),
        points: 3
    }
}

fn get_a_rand_dev(rands: Vec<i32>, numbers: &Vec<i32>) -> Development {
    assert_eq!(rands.len(), numbers.len());
    let mut entries = vec![];
    // the last element in rands indicates the resource type of the dev
    // the last element in numbers indicates the point of the dev
    for i in 0..numbers.len() - 1 {
        entries.push(ResourceEntry { resource_type: rands[i], number: numbers[i] });
    }
    Development {
        price: Some(ResourceMap { entries }),
        resource_type: *rands.last().unwrap(),
        points: *numbers.last().unwrap()
    }
}

fn get_rand_devs(templates: Vec<Vec<i32>>, num: usize) -> Vec<Development> {
    let mut dev_cards = vec![];
    for _ in 0..num {
        // choose a random template
        let index = rand::thread_rng().gen_range(0..templates.len());
        // an element in templates is numbers vec
        let numbers = &templates[index];
        // a dev card's price contains at most 4 different resource types 
        // and 1 entra element as point
        assert!(numbers.len() <= 5);
        let rands = get_multi_rand_nums(
            numbers.len(), ResourceType::Agate as i32, ResourceType::Sapphire as i32);
        dev_cards.push(get_a_rand_dev(rands, numbers));
    }
    dev_cards
}

fn get_default_resource_map() -> ResourceMap {
    ResourceMap {
        entries: vec![
            ResourceEntry { resource_type: ResourceType::Agate as i32, number: 0 },
            ResourceEntry { resource_type: ResourceType::Emerald as i32, number: 0 },
            ResourceEntry { resource_type: ResourceType::Diamond as i32, number: 0 },
            ResourceEntry { resource_type: ResourceType::Ruby as i32, number: 0 },
            ResourceEntry { resource_type: ResourceType::Sapphire as i32, number: 0 },
        ]
    }
}