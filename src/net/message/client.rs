use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::game::cards::Card;
use actix::prelude::*;
use actix::Message;
use crate::net::message::server::ServerMessage;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    PlayCard { card: Card },
    DrawCard,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMessage {
    pub player: Uuid,
    pub message_type: MessageType
}

impl Message for ClientMessage {
    type Result = String;
}
