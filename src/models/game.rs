use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::models::cards::{Card, CardStack};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Player {
    id: Uuid,
    name: String,
    hand: Vec<Card>,
    score_stack: ScoreStack
}

impl Player {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            hand: Vec::new(),
            score_stack: ScoreStack::new()
        }
    }

    pub fn hand_mut(&mut self) -> &mut Vec<Card> {
        &mut self.hand
    }

    pub fn score_stack_mut(&mut self) -> &mut ScoreStack {
        &mut self.score_stack
    }
}

#[derive(Debug)]
pub struct ScoreStack {
    matches: Vec<Match>
}

impl ScoreStack {
    pub fn new() -> Self {
        Self { matches: Vec::new() }
    }

    pub fn push(&mut self, item: Match) {
        self.matches.push(item);
    }

    pub fn count(&self) -> u32 {
        self.matches
            .iter()
            .copied()
            .fold(0, |sum, x| sum + x.points())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Match {
    Full,
    Half
}

impl Match {
    pub fn points(self) -> u32 {
        match self {
            Match::Full => 3,
            Match::Half => 1
        }
    }
}

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
        self.players.insert(player.id, player);
    }

    pub fn next_player(&mut self) {
        unimplemented!()
    }
}