use crate::models::cards::Card::Joker;
use crate::models::cards::{
    Card, CardStack, Color, DoorType, JokerType, MotorType, PartType, TireType, WindowType,
};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Player {
    id: Uuid,
    name: String,
    hand: Vec<Card>,
    score_stack: ScoreStack,
}

impl Player {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            hand: Vec::new(),
            score_stack: ScoreStack::new(),
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
    matches: Vec<Match>,
}

impl ScoreStack {
    pub fn new() -> Self {
        Self {
            matches: Vec::new(),
        }
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
    Half,
}

impl Match {
    pub fn points(self) -> u32 {
        match self {
            Match::Full => 3,
            Match::Half => 1,
        }
    }
}

pub struct Desk {
    card_stack: CardStack,
    players: HashMap<Uuid, Player>,
    current_player: Uuid,
}

impl Desk {
    pub fn new() -> Self {
        Self {
            card_stack: CardStack::new(),
            players: HashMap::new(),
            current_player: Uuid::nil(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id, player);
    }

    pub fn next_player(&mut self) {
        unimplemented!()
    }
}

pub struct Contract {
    contract_parts: Vec<Card>,
}

impl Contract {
    pub fn new(parts: Vec<Card>) -> Self {
        Self {
            contract_parts: parts,
        }
    }

    pub fn generate() -> Self {
        let mut contract_parts = Vec::new();
        let range = rand::thread_rng().gen_range(2, 8);
        let part_list = CardStack::new()
            .inner()
            .iter()
            .copied()
            .filter(|card| match *card {
                Joker(_) => false,
                _ => true,
            })
            .collect::<Vec<Card>>();

        for _ in 0..range {
            'part_loop: for part in &part_list {
                let new_part_type: PartType;

                match part {
                    Card::Part(part_type) => {
                        new_part_type = *part_type;
                    }
                    _ => unreachable!(),
                }

                'existing_loop: for existing in &contract_parts {
                    match *existing {
                        Card::Part(part_type) => match part_type {
                            PartType::Tires(tire_type) => match new_part_type {
                                PartType::Tires(new_tire_type) => {
                                    if tire_type != new_tire_type {
                                        continue 'part_loop;
                                    }
                                }
                                _ => continue 'existing_loop,
                            },
                            PartType::Motor(motor_type) => match new_part_type {
                                PartType::Motor(new_motor_type) => {
                                    continue 'part_loop;
                                }
                                _ => continue 'existing_loop,
                            },
                            PartType::Door(door_type) => match new_part_type {
                                PartType::Door(new_door_type) => {}
                                _ => continue 'existing_loop,
                            },
                            PartType::Window(window_type) => match new_part_type {
                                PartType::Window(new_window_type) => {
                                    if window_type == new_window_type
                                        && window_type != WindowType::Side
                                    {
                                        continue 'part_loop;
                                    }
                                }
                                _ => continue 'existing_loop,
                            },
                            PartType::Paint(paint_type) => match new_part_type {
                                PartType::Motor(new_motor_type) => {
                                    continue 'part_loop;
                                }
                                _ => continue 'existing_loop,
                            },
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
