use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::models::cards::Card;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Player {
    id: Uuid,
    name: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            hand: Vec::new()
        }
    }
}

pub struct ScoreStack {
    matches: Vec<Match>
}

impl ScoreStack {
    pub fn new() -> Self {
        Self { matches: Vec::new() }
    }

    pub fn count(&self) -> u32 {
        self.matches
            .iter()
            .copied()
            .fold(0, |sum, x| sum + x.points())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
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
    players: Vec<Player>
}