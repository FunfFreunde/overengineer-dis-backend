use crate::game::cards::Card;
use crate::net::message::server::ServerMessage;
use actix::prelude::*;
use actix::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    PlayCard { card: Card },
    DrawCard,
    Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMessage {
    pub player: Uuid,
    pub message_type: MessageType,
}

impl Message for ClientMessage {
    type Result = String;
}
