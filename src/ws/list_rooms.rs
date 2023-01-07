use actix::prelude::*;

use super::Server;

#[derive(Clone, Message)]
#[rtype(result = "Vec<String>")]
pub struct ListRooms;

impl Handler<ListRooms> for Server {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.rooms.keys().cloned().collect())
    }
}