use serde::{Deserialize, Serialize};
use crate::models::cards::Card::Part;
use serde::export::fmt::Display;

const INSTANCES_PER_CARD: usize = 4;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "card_type")]
pub enum Card {
    Part(PartType),
    Joker(JokerType)
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct CardStack {
    cards: Vec<Card>,
}

impl CardStack {
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for _ in 0..INSTANCES_PER_CARD {
            cards.push(Card::Part(PartType::Tires(TireType::Summer)));
            cards.push(Card::Part(PartType::Tires(TireType::Winter)));
            cards.push(Card::Part(PartType::Tires(TireType::Generic)));

            cards.push(Card::Part(PartType::Motor(MotorType::Gasoline)));
            cards.push(Card::Part(PartType::Motor(MotorType::Electric)));
            cards.push(Card::Part(PartType::Motor(MotorType::Diesel)));

            cards.push(Card::Part(PartType::Door(DoorType::Front)));
            cards.push(Card::Part(PartType::Door(DoorType::Back)));
            cards.push(Card::Part(PartType::Door(DoorType::Slide)));

            cards.push(Card::Part(PartType::Window(WindowType::Front)));
            cards.push(Card::Part(PartType::Window(WindowType::Back)));
            cards.push(Card::Part(PartType::Window(WindowType::Side)));

            cards.push(Card::Part(PartType::Paint(Color::Blue)));
            cards.push(Card::Part(PartType::Paint(Color::Red)));
            cards.push(Card::Part(PartType::Paint(Color::Green)));
            cards.push(Card::Part(PartType::Paint(Color::Yellow)));

            cards.push(Card::Joker(JokerType::Intern));
            cards.push(Card::Joker(JokerType::Cancellation));
            cards.push(Card::Joker(JokerType::ShoddyWork { min: 0, max: 10}))
        }

        Self { cards }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "joker_type")]
pub enum JokerType {
    Intern,
    ShoddyWork { min: u8, max: u8 },
    Cancellation
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "part_type")]
pub enum PartType {
    Motor(MotorType),
    Door(DoorType),
    Window(WindowType),
    Tires(TireType),
    Paint(Color),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "specialization")]
pub enum MotorType {
    Electric,
    Diesel,
    Gasoline,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "specialization")]
pub enum DoorType {
    Front,
    Back,
    Slide,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "specialization")]
pub enum WindowType {
    Front,
    Back,
    Side,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "specialization")]
pub enum TireType {
    Winter,
    Summer,
    Generic,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "specialization")]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}
