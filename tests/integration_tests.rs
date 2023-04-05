use chatroom_example::chat::{Room, WsChatSession};

#[actix::test]
async fn test_ws_chat_session_interaction() {
    let room = Room::new("test_room".to_string(), "password123".to_string());
    let ws_chat_session = WsChatSession::new(room.clone());

    assert!(room.verify_password("password123"));

    // Ensure the session's room is the same as the original room
    assert_eq!(ws_chat_session.room.id, room.id);
    assert_eq!(ws_chat_session.room.password, room.password);
}
