# actix-chatroom-example

When a new WebSocket connection request comes in, the following sequence of events occurs:

1. The request is received by the Actix Web server.

2. The server routes the request to the chat_route function in main.rs based on the URL pattern /ws/{room_id}/{password}.

3. The chat_route function extracts the room_id and password from the URL path.

4. It then looks up the corresponding room in the rooms HashMap. If the room does not exist, it creates a new one.

5. The chat_route function then checks if the provided password is correct using the verify_password method of the Room struct.

6. If the password is correct, it creates a new WsChatSession actor instance using the WsChatSession::new method with the associated room.

7. The ws::start function is called with the created WsChatSession instance, along with the original HttpRequest and Payload. This function initializes the WebSocket connection, upgrades the HTTP request to a WebSocket connection, and starts the WsChatSession actor.

8. When the WsChatSession actor starts, the Actor for WsChatSession trait implementation's started method is called. It adds the actor's address to the list of sessions in the Room.

9. The WebSocket connection is now active, and the WsChatSession actor is running. The ws::StreamHandler implementation for WsChatSession handles incoming WebSocket messages. If a chat event (e.g., a message) is received, it is processed and sent to all other sessions in the room.

10. When the WebSocket connection is closed, the WsChatSession actor's stopping method is called. This method removes the actor's address from the list of sessions in the Room.

11. Finally, the WebSocket connection is closed, and the WsChatSession actor is stopped.

This is the sequence of events that occur when a new request comes in, and the functions called at each step.
