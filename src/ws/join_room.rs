use actix::prelude::*;

use super::ChatMessage;
use super::Server;

#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct JoinRoom(pub String, pub Option<String>, pub Recipient<ChatMessage>);

impl Handler<JoinRoom> for Server {
    type Result = MessageResult<JoinRoom>;

    fn handle(&mut self, message: JoinRoom, _ctx: &mut Self::Context) -> Self::Result {
        let JoinRoom(room_name, client_name, client) = message;

        let id = self.add_client_to_room(&room_name, None, client);
        let join_message = format!(
            "{} joined {}",
            client_name.unwrap_or_else(|| "anon".to_string()),
            room_name
        );

        self.send_chat_message(&room_name, &join_message, id);
        MessageResult(id)
    }
}