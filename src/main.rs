use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod chat;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rooms = Arc::new(Mutex::new(HashMap::<String, chat::Room>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(rooms.clone())
            .route("/ws/{room_id}/{password}", web::get().to(chat_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn chat_route(
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
