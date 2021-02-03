use std::collections::VecDeque;
use std::mem;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::uno_pb::{Card, CardColor, CardText};

pub struct Deck {
    pile: VecDeque<Card>,
    discard_pile: VecDeque<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck {
            pile: VecDeque::new(),
            discard_pile: VecDeque::new()
        };
        deck.init();
        deck
    }

    fn init(&mut self) {
        self.pile.clear();
        self.discard_pile.clear();

        let non_wild_colors = [
            CardColor::Red as i32, CardColor::Yellow as i32, 
            CardColor::Green as i32, CardColor::Blue as i32];
        let non_wild_texts = [
            CardText::Zero as i32, CardText::One as i32, 
            CardText::Two as i32, CardText::Three as i32,
            CardText::Four as i32, CardText::Five as i32, 
            CardText::Six as i32, CardText::Seven as i32,
            CardText::Eight as i32, CardText::Nine as i32, 
            CardText::Skip as i32, CardText::Reverse as i32,
            CardText::DrawTwo as i32];
        for color in non_wild_colors.iter() {
            for text in non_wild_texts.iter() {
                self.pile.push_front(Card{ color: *color, text: *text });
                if *text != CardText::Zero as i32 {
                    self.pile.push_front(Card{ color: *color, text: *text });
                }
            }
        }

        let wild_card_num = 4;
        for _ in 0..wild_card_num {
            self.pile.push_front(Card{ 
                color: CardColor::Black as i32, 
                text: CardText::Wild as i32
            });
            self.pile.push_front(Card{ 
                color: CardColor::Black as i32, 
                text: CardText::DrawFour as i32
            });
        }
        self.pile.make_contiguous().shuffle(&mut thread_rng());
    }

    pub fn deal_init_handcards(&mut self, player_num: usize) -> Vec<Vec<Card>> {
        let mut init_handcards = vec![];
        init_handcards.resize(player_num, vec![]);
        for _ in 0..7 {
            for player in 0..player_num {
                init_handcards[player].push(self.draw());
            }
        }
        init_handcards
    }

    pub fn draw(&mut self) -> Card {
        if self.pile.is_empty() {
            mem::swap(&mut self.pile, &mut self.discard_pile);
            self.pile.make_contiguous().shuffle(&mut thread_rng());
        }
        self.pile.pop_front().unwrap()
    }

    pub fn draw_num(&mut self, num: i32) -> Vec<Card> {
        let mut cards = Vec::with_capacity(num as usize);
        for _ in 0..num {
            cards.push(self.draw());
        }
        cards
    }

    pub fn put_to_bottom(&mut self, card: Card) {
        self.pile.push_back(card);
    }

    pub fn discard(&mut self, card: Card) {
        self.discard_pile.push_front(card);
    }
}
