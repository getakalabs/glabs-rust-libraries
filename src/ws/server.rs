use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use std::collections::HashMap;

use super::ChatMessage;
use super::Client;
use super::LeaveRoom;
use super::Room;
use super::SendMessage;

#[derive(Default)]
pub struct Server {
    pub rooms: HashMap<String, Room>,
}

// WS server implementation
impl Server {
    /// Take room
    pub fn take_room<T: Into<String>>(&mut self, room_name: T) -> Option<Room> {
        // Set bindings
        let bindings = room_name.into();

        // Return room if valid
        self.rooms
            .get_mut(&bindings)
            .map_or(None, |r| {
                Some(std::mem::take(r))
            })
    }

    /// Add client to a room
    pub fn add_client_to_room<T: Into<String>>(&mut self, room_name: T, id: Option<usize>, client: Client) -> usize {
        let bindings = room_name.into();
        let mut id = id.unwrap_or_else(rand::random::<usize>);

        if let Some(room) = self.rooms.get_mut(&bindings) {
            loop {
                if room.contains_key(&id) {
                    id = rand::random::<usize>();
                } else {
                    break;
                }
            }

            room.insert(id, client);
            return id;
        }

        // Create a new room for the first client
        let mut room: Room = HashMap::new();
        room.insert(id, client);
        self.rooms.insert(bindings.to_owned(), room);

        id
    }

    /// Send chat message
    pub fn send_chat_message<RN, M>(&mut self, room_name: RN, message: M, _src: usize) -> Option<()>
        where RN: Into<String>,
              M: Into<String>
    {
        // Create bindings
        let room_name_bindings = room_name.into();
        let message_bindings = message.into();

        // Check room
        let room = self.take_room(room_name_bindings.clone());
        if room.is_none() {
            return None;
        }

        // Shadow room
        let mut room = room.unwrap();
        for (id, client) in room.drain() {
            if client.try_send(ChatMessage(message_bindings.to_owned())).is_ok() {
                self.add_client_to_room(room_name_bindings.clone(), Some(id), client);
            }
        }

        // Return some
        Some(())
    }
}

impl Actor for Server {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<LeaveRoom>(ctx);
        self.subscribe_system_async::<SendMessage>(ctx);
    }
}

impl SystemService for Server {}
impl Supervised for Server {}
