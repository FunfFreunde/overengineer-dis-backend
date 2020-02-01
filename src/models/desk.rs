use crate::models::cards::{Card, PartType, MotorType, JokerType, TireType, DoorType, Color, WindowType, CardStack};	
use crate::models::cards::Card::Joker;
use std::collections::HashMap;
use uuid::Uuid;
use crate::models::player::Player;
use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

pub struct Desk {
    card_stack: CardStack,
    players: Vec<Player>,
    current_player: usize
}

impl Desk {
    pub fn new() -> Self {
        Self {
            card_stack: CardStack::new(),
            players: Vec::new(),
            current_player: 0
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Contract {
    contract_parts: Vec<Card>
}

impl Contract {
    pub fn new(parts:Vec<Card>) -> Self {
        Self {
            contract_parts: parts
        }
    }

    pub fn generate() -> Self{
        let mut contract_parts = Vec::new();
        let range = rand::thread_rng().gen_range(2, 8);
        let part_list = CardStack::new()
        .inner()
        .iter()
        .copied()
        .filter(|card| {
            match *card {
                Joker(_) => false,
                _ => true
            }
        })
        .collect::<Vec<Card>>();
        
        for _ in 0..range {
            'part_loop: for part in &part_list {
                let new_part_type: PartType;

                match part {
                    Card::Part(part_type) => {
                        new_part_type = *part_type;
                    },
                    _ => unreachable!()
                }

                'existing_loop: for existing in &contract_parts {
                    match *existing {
                        Card::Part(part_type) => {
                            match part_type {
                                PartType::Tires(tire_type) =>{
                                    match new_part_type {
                                        PartType::Tires(new_tire_type) => {
                                            if tire_type != new_tire_type {
                                                continue 'part_loop;
                                            }
                                        },
                                        _ => continue 'existing_loop
                                    }
                                },
                                PartType::Motor(motor_type) =>{
                                    match new_part_type {
                                        PartType::Motor(new_motor_type) => {
                                            continue 'part_loop;
                                        },
                                        _ => continue 'existing_loop
                                    }
                                },
                                PartType::Door(door_type) =>{
                                    match new_part_type {
                                        PartType::Door(new_door_type) => {
                                        },
                                        _ => continue 'existing_loop
                                    }
                                },
                                PartType::Window(window_type) =>{
                                    match new_part_type {
                                        PartType::Window(new_window_type) => {
                                            if window_type == new_window_type && window_type != WindowType::Side{
                                                continue 'part_loop;
                                            }
                                        },
                                        _ => continue 'existing_loop
                                    }
                                    
                                },
                                PartType::Paint(paint_type) =>{
                                    match new_part_type {
                                        PartType::Motor(new_motor_type) => {
                                            continue 'part_loop;
                                        },
                                        _ => continue 'existing_loop
                                    }
                                }
                            }

                        },
                        _ => {
                            continue 'existing_loop;
                        }
                    }
                }
                contract_parts.push(*part)
            }
        }
        return Contract::new(contract_parts);
    }
}