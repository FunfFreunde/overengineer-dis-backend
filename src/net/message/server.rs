use crate::game::cards::Card;
use crate::game::contract::Contract;
use actix::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    Accept,
    NotYourTurn,
    WrongCard,
    PlayerChange { player: Uuid },
    ContractChange { contract: Contract },
    Deal { cards: Vec<Card> },
}

impl Message for ServerMessage {
    type Result = ();
}
