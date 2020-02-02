use crate::game::cards::Card::Joker;
use crate::game::cards::{
    Card, CardStack, Color, DoorType, JokerType, MotorType, PartType, TireType, WindowType,
};
use crate::game::contract::Contract;
use crate::game::player::Player;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use std::ops::Deref;

pub struct Desk {
    card_stack: CardStack,
    contract: Contract,
    players: Vec<Player>,
    current_player: usize,
}

impl Desk {
    pub fn new() -> Self {
        Self {
            card_stack: CardStack::new(),
            contract: Contract::generate(),
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

    pub fn current_player_mut(&mut self) -> Option<&mut Player> {
        self.players.get_mut(self.current_player)
    }

    pub fn next_player(&mut self) {
        if self.current_player < self.players.len() - 1 {
            self.current_player += 1;
        } else {
            self.current_player = 0;
        }
    }

    pub fn remove_part_from_contract(&mut self, index: usize) {
        self.contract.contract_parts_mut().remove(index);
    }

    pub fn play_card(&mut self, player_id: Uuid, card: &Card) {
//        let player = match self.current_player() {
//            Some(player) => player,
//            None => return Err("player not found".into())
//        };
//
//        if player.id() != &player_id {
//            return Err("not your turn".into());
//        }
//
//        let card = match player
//            .hand()
//            .iter()
//            .find(|given| &card == given) {
//
//            Some(index) => index,
//            None => return Err("card not found in player hand".into())
//        };
//
//        match card {
//            Card::Part(part_type) => {
//                let contract_part_index = match self
//                    .contract
//                    .contract_parts_mut()
//                    .iter()
//                    .position(|x| x == card) {
//
//                    Some(index) => index,
//                    None => return Err("card not found in contract".into())
//                };
//
//                let pmut = self.current_player_mut();
//            },
//            _ => {}
//        }
//
//        Ok("".into())
//
////        //Actual play logic
////        match card {
////            Card::Part(part_type) => {
////                let index_contract_part_optional = self
////                    .contract
////                    .contract_parts_mut()
////                    .iter()
////                    .position(|x| x == card);
////                let index_contract_part: usize;
////                match index_contract_part_optional {
////                    None => return String::from("reject_card_not_exists"),
////                    Some(contract_part) => {
////                        self.remove_part_from_contract(contract_part);
////                        player.hand_mut().remove(index);
////                    }
////                }
////            }
////            Card::Joker(joker_type) => {
////                // TODO: Handle special cases
////                player.hand_mut().remove(index);
////            }
////        }
////
////        return String::from("accept");
    }
}
