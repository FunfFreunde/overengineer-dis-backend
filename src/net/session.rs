use std::time::Instant;
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web::{web, HttpRequest, HttpResponse, Error};
use std::time::Duration;
use crate::net::message::client::{ClientMessage, MessageType};
use crate::game::desk::Desk;
use std::collections::HashMap;
use uuid::Uuid;
use crate::net::message::server::ServerMessage;
use crate::game::player::Player;
use actix::dev::MessageResponse;
use std::sync::Arc;
use std::cell::Cell;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct GameServer {
    id: Uuid,
    desk: Desk,
    clients: HashMap<Uuid, Addr<GameSocket>>
}

impl GameServer {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            desk: Desk::new(),
            clients: HashMap::new()
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn create_player(&mut self, name: &str) -> Uuid {
        let uuid = Uuid::new_v4();
        let player = Player::new(uuid.clone(), name);

        self.desk.add_player(player);
        uuid
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<ClientMessage> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        let player = msg.player;

        if let Some(socket) = self.clients.get(&player) {
            match msg.message_type {
                MessageType::PlayCard { card } => {
                    socket.do_send(ServerMessage::Accept);
                },
                MessageType::DrawCard => {
                    socket.do_send(ServerMessage::Accept);
                }
            }
        }
    }
}

pub struct GameSocket {
    uuid: Uuid,
    hb: Instant,
    server: Arc<Addr<GameServer>>
}

impl Actor for GameSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text(self.uuid.to_string());
        self.hb(ctx);
    }
}

impl GameSocket {
    pub fn new(uuid: Uuid, server_addr: Arc<Addr<GameServer>>) -> Self {
        Self { uuid, hb: Instant::now(), server: server_addr}
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
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(message) => {
                match message {
                    ws::Message::Text(txt) => {
                        let message = serde_json::from_str::<ClientMessage>(&txt);

                        match message {
                            Ok(msg) => {
                                println!("received valid message");
                                if let Err(e) = self.server.try_send(msg) {
                                    println!("send error: {}", e);
                                }
                            },
                            Err(e) => { eprintln!("received invalid invalid json from the client: {}", e) }
                        }
                    },
                    ws::Message::Pong(_) => {
                        self.hb = Instant::now();
                    },
                    _ => eprintln!("unsupported message type")
                }
            },
            Err(e) => eprintln!("protocol error: {}", e)
        }
    }
}

impl Handler<ServerMessage>for GameSocket {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("hemlo");
        ctx.text("hemlo");
    }
}