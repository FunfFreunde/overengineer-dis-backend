use crate::models::cards::Card::Joker;
use crate::models::cards::PartType::Motor;
use crate::models::cards::{Card, CardStack, JokerType, MotorType, PartType, TireType};
use crate::models::contract::Contract;
use actix_web::web::Json;
use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

mod models;

async fn index() -> impl Responder {
    let contract = Contract::generate();

    web::Json(contract)
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard =
        sentry::init("https://7bb10419f3bf43f2a79ede13560b7cdf@sentry.fsn.k8s.ciapa.tech/4");
    env::set_var("RUST_BACKTRACE", "1");
    sentry::integrations::panic::register_panic_handler();

    dotenv().ok();

    HttpServer::new(|| App::new().route("/backend/index", web::get().to(index)))
        .bind(env::var("LISTEN_ADDRESS")?)?
        .run()
        .await?;

    Ok(())
}
