use crate::game::cards::Card::Joker;
use crate::game::cards::{
    Card, CardStack, Color, DoorType, JokerType, MotorType, PartType, TireType, WindowType,
};
use crate::game::player::Player;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub struct Desk {
    card_stack: CardStack,
    players: Vec<Player>,
    current_player: usize,
}

impl Desk {
    pub fn new() -> Self {
        Self {
            card_stack: CardStack::new(),
            players: Vec::new(),
            current_player: 0,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player)
    }

    pub fn current_player(&self) -> Option<&Player> {
        self.players.get(self.current_player)
    }

    pub fn current_player_mut(&mut self) -> Option<&Player> {
        self.players.get(self.current_player)
    }

    pub fn next_player(&mut self) {
        if self.current_player < self.players.len() - 1 {
            self.current_player += 1;
        } else {
            self.current_player = 0;
        }
    }
}