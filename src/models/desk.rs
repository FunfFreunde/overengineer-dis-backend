use crate::models::cards::CardStack;
use std::collections::HashMap;
use uuid::Uuid;
use crate::models::game::Player;

pub struct Desk {
    card_stack: CardStack,
    players: HashMap<Uuid, Player>,
    current_player: Uuid
}

impl Desk {
    pub fn new() -> Self {
        Self {
            card_stack: CardStack::new(),
            players: HashMap::new(),
            current_player: Uuid::nil()
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id(), player);
    }

    pub fn next_player(&mut self) {
        unimplemented!()
    }
}