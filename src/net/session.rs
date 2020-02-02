use crate::game::desk::Desk;
use crate::game::player::Player;
use crate::net::message::client::{ClientMessage, MessageType};
use crate::net::message::server::ServerMessage;
use actix::dev::MessageResponse;
use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::cell::Cell;
use std::collections::HashMap;
use std::future::Future;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Poll, RawWakerVTable, Waker};
use std::time::Duration;
use std::time::Instant;
use uuid::Uuid;
use crate::game::cards::CardStack;
use crate::game::contract::Contract;
use actix_web::web::Json;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct GameServer {
    id: Uuid,
    desk: Desk,
}

impl GameServer {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            desk: Desk::new(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<ClientMessage> for GameServer {
    type Result = String;

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg.message_type {
            MessageType::PlayCard { card } => {
                serde_json::to_string_pretty(&ServerMessage::Accept).unwrap()
            }
            MessageType::DrawCard => serde_json::to_string_pretty(&ServerMessage::Accept).unwrap(),
            MessageType::Status => serde_json::to_string_pretty(&ServerMessage::PlayerChange {
                player: Uuid::new_v4(),
            })
            .unwrap(),
        }
    }
}

pub struct GameSocket {
    uuid: Uuid,
    hb: Instant,
    server: Arc<Addr<GameServer>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IdMsg {
    id: String
}

impl Actor for GameSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let mut stack = CardStack::new();
        let id_msg = IdMsg { id: self.uuid.to_string() };

        ctx.text(serde_json::to_string_pretty(&id_msg).unwrap());
        ctx.text(serde_json::to_string_pretty(&stack.deal(8).unwrap()).unwrap());
        ctx.text(serde_json::to_string_pretty(&Contract::generate()).unwrap());
        self.hb(ctx);
    }
}

impl GameSocket {
    pub fn new(uuid: Uuid, server_addr: Arc<Addr<GameServer>>) -> Self {
        Self {
            uuid,
            hb: Instant::now(),
            server: server_addr,
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(message) => match message {
                ws::Message::Text(txt) => {
                    let message = serde_json::from_str::<ClientMessage>(&txt);

                    match message {
                        Ok(msg) => {
                            println!("received valid message");

                            let res = self.server.send(msg);

                            let future = async {
                                match res.await {
                                    Ok(answer) => {
                                        ctx.text(answer);
                                    }
                                    Err(e) => {
                                        eprintln!("error sending message: {}", e);
                                    }
                                }
                            };

                            futures::executor::block_on(future);
                        }
                        Err(e) => eprintln!("received invalid invalid json from the client: {}", e),
                    }
                }
                ws::Message::Pong(_) => {
                    println!("received pong from player {}", &self.uuid);
                    self.hb = Instant::now();

//                    let res = self.server.send(ClientMessage {
//                        player: self.uuid,
//                        message_type: MessageType::Status,
//                    });
//
//                    let future = async {
//                        match res.await {
//                            Ok(answer) => {
//                                ctx.text(answer);
//                            }
//                            Err(e) => {
//                                eprintln!("error sending message: {}", e);
//                            }
//                        }
//                    };

                    //futures::executor::block_on(future);
                }
                _ => eprintln!("unsupported message type"),
            },
            Err(e) => eprintln!("protocol error: {}", e),
        }
    }
}
