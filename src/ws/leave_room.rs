use actix::prelude::*;

use super::Server;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub usize);

impl Handler<LeaveRoom> for Server {
    type Result = ();

    fn handle(&mut self, message: LeaveRoom, _ctx: &mut Self::Context) {
        if let Some(room) = self.rooms.get_mut(&message.0) {
            room.remove(&message.1);
        }
    }
}