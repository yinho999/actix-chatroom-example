use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Clone)]
pub struct Room {
    pub id: String,
    pub password: Arc<String>,
    sessions: Arc<Mutex<Vec<Addr<WsChatSession>>>>,
}

impl Room {
    pub fn new(id: String, password: String) -> Self {
        Room {
            id,
            password: Arc::new(password),
            sessions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        &*self.password == password
    }
}

pub struct WsChatSession {
    room: Room,
    addr: Option<Addr<Self>>,
}

impl WsChatSession {
    pub fn new(room: Room) -> Self {
        WsChatSession { room, addr: None }
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address());
        let mut sessions = self.room.sessions.lock().unwrap();
        sessions.push(ctx.address());
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        if let Some(addr) = &self.addr {
            let mut sessions = self.room.sessions.lock().unwrap();
            sessions.retain(|a| a != addr);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ChatEvent {
    Message { user: String, content: String },
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let event: Result<ChatEvent, serde_json::Error> = serde_json::from_str(&text);
                match event {
                    Ok(ChatEvent::Message { user, content }) => {
                        let message = format!("{}: {}", user, content);
                        let _ = self
                            .room
                            .sessions
                            .lock()
                            .unwrap()
                            .iter()
                            .filter(|a| a != &self.addr.as_ref().unwrap())
                            .for_each(|addr| {
                                let _ = addr.do_send(WsMessage(message.clone()));
                            });
                        ctx.text(message);
                    }
                    Err(_) => {
                        ctx.text("Invalid chat event format");
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {
                ctx.stop();
            }
        }
    }
}

impl Handler<WsMessage> for WsChatSession {
    type Result = ();
    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
