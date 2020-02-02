use uuid::Uuid;
use crate::game::cards::Card;
use crate::game::contract::Contract;
use serde::{Deserialize, Serialize};
use actix::Message;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    Accept,
    NotYourTurn,
    WrongCard,
    PlayerChange { player: Uuid },
    ContractChange { contract: Contract },
    Deal { cards: Vec<Card> }
}

impl Message for ServerMessage {
    type Result = ();
}