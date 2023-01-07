use actix::prelude::*;

use super::Session;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

impl Handler<ChatMessage> for Session {
    type Result = ();

    fn handle(&mut self, message: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(message.0);
    }
}