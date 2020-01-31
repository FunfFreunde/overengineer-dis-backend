use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use actix_web::web::Json;
use crate::models::cards::{Card, PartType, MotorType, JokerType, TireType, CardStack};
use crate::models::cards::PartType::Motor;
use crate::models::cards::Card::Joker;

mod models;

async fn index() -> impl Responder {
    web::Json(CardStack::new())
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    HttpServer::new(|| App::new()
        .route("/", web::get().to(index))
    )
        .bind(env::var("LISTEN_ADDRESS")?)?
        .run()
        .await?;

    Ok(())
}
