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


pub struct Desk {
    players: Vec<Player>
}