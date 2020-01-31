use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "card_type")]
pub enum Card {
    Part(PartType)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "part_type")]
pub enum PartType {
    Motor(MotorType),
    Door(DoorType),
    Window(WindowType),
    Tires(TireType),
    Paint(Color),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "specialization")]
pub enum MotorType {
    Electric,
    Diesel,
    Gasoline,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "specialization")]
pub enum DoorType {
    Front,
    Back,
    Slide,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "specialization")]
pub enum WindowType {
    Front,
    Back,
    Side,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "specialization")]
pub enum TireType {
    Winter,
    Summer,
    Generic,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "specialization")]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}
