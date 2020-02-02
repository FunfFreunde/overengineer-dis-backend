use crate::net::message::client::{ClientMessage, MessageType};
use crate::net::session::{GameServer, GameSocket};
use actix::{Actor, Addr};
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::ops::Deref;
use std::sync::Arc;
use uuid::Uuid;

mod game;
mod net;

/// do websocket handshake and start `MyWebSocket` actor
async fn ws_index(
    data: web::Data<Addr<GameServer>>,
    r: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let uuid = Uuid::new_v4();
    let game_socket = GameSocket::new(uuid, data.into_inner());

    let res = ws::start(game_socket, &r, stream);
    res
}

async fn index() -> impl Responder {
    web::Json(ClientMessage {
        player: Uuid::new_v4(),
        message_type: MessageType::DrawCard,
    })
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard =
        sentry::init("https://7bb10419f3bf43f2a79ede13560b7cdf@sentry.fsn.k8s.ciapa.tech/4");
    env::set_var("RUST_BACKTRACE", "1");
    sentry::integrations::panic::register_panic_handler();

    dotenv().ok();

    let game_server = GameServer::new();
    let data = game_server.start();

    HttpServer::new(move || App::new()
        .data(data.clone())
        .wrap(middleware::Logger::default())
        .route("/backend/join", web::get().to(ws_index))
        .route("/backend/", web::get().to(index))
    )
        .bind(env::var("LISTEN_ADDRESS")?)?
        .run()
        .await?;

    Ok(())
}
