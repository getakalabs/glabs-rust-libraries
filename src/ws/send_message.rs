use actix::prelude::*;

use super::Server;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendMessage(pub String, pub usize, pub String);

impl Handler<SendMessage> for Server {
    type Result = ();

    fn handle(&mut self, message: SendMessage, _ctx: &mut Self::Context) {
        let SendMessage(room_name, id, message) = message;
        self.send_chat_message(&room_name, &message, id);
    }
}