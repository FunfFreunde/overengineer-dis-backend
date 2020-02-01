use crate::game::cards::Card::Joker;
use crate::game::cards::PartType::Motor;
use crate::game::cards::{Card, CardStack, JokerType, MotorType, PartType, TireType};
use crate::game::contract::Contract;
use actix_web::web::Json;
use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

mod game;

#[derive(Deserialize)]
struct JoinInfo {
    username: String,
    uuid: Uuid
}

#[derive(Deserialize)]
struct LeaveInfo {
    uuid: Uuid
}

#[derive(Deserialize)]
struct NewInfo {
    username: String
}

async fn index() -> impl Responder {
    let contract = Contract::generate();

    web::Json(contract)
}

async fn get_games() -> impl Responder {
    let contract = Contract::generate();

    web::Json(contract)
}

async fn get_games_join(info: web::Path<JoinInfo>) -> String {
    format!("User {} joined room {}", info.username, info.uuid)
}

async fn get_games_new(info: web::Path<NewInfo>) -> String {
    format!("User {} created new room", info.username)
}

async fn get_games_leave(info: web::Path<LeaveInfo>) -> String {
    format!("Someone left room {}", info.uuid)
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard =
        sentry::init("https://7bb10419f3bf43f2a79ede13560b7cdf@sentry.fsn.k8s.ciapa.tech/4");
    env::set_var("RUST_BACKTRACE", "1");
    sentry::integrations::panic::register_panic_handler();

    dotenv().ok();

    HttpServer::new(|| App::new()
            .service(web::resource("/backend/index").route(web::get().to(index)))
            .service(web::resource("/backend/games").route(web::get().to(get_games)))
            .service(web::resource("/backend/games/{uuid}/join/{username}").route(web::get().to(get_games_join)))
            .service(web::resource("/backend/games/new/{username}").route(web::get().to(get_games_new)))
            .service(web::resource("/backend/games/{uuid}/leave").route(web::get().to(get_games_leave)))
        )
        .bind(env::var("LISTEN_ADDRESS")?)?
        .run()
        .await?;

    Ok(())
}
