use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use chatroom_example::{chat, chat_route};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
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
