use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub mod chat;
pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    info: web::Path<(String, String)>,
    rooms: web::Data<Arc<Mutex<HashMap<String, chat::Room>>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let (room_id, password) = info.into_inner();
    let room = {
        let mut rooms = rooms.lock().unwrap();
        rooms
            .entry(room_id.clone())
            .or_insert_with(|| chat::Room::new(room_id.clone(), password.clone()))
            .clone()
    };
    if room.verify_password(&password) {
        ws::start(chat::WsChatSession::new(room), &req, stream)
    } else {
        Err(actix_web::error::ErrorForbidden("Invalid password"))
    }
}
