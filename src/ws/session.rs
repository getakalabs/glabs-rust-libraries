use actix::prelude::*;
use actix_broker::BrokerIssue;
use actix_web_actors::ws;

use super::JoinRoom;
use super::LeaveRoom;
use super::ListRooms;
use super::SendMessage;
use super::Server;

#[derive(Default)]
pub struct Session {
    id: usize,
    room: String,
    name: Option<String>,
}

impl Session {
    pub fn join_room<T: Into<String>>(&mut self, room_name: T, ctx: &mut ws::WebsocketContext<Self>) {
        // Create bindings
        let room_name = room_name.into().to_owned();

        // First send a leave message for the current room
        let leave_message = LeaveRoom(self.room.clone(), self.id);

        // issue_sync comes from having the `BrokerIssue` trait in scope.
        self.issue_system_sync(leave_message, ctx);

        // Then send a join message for the new room
        let join_message = JoinRoom(
            room_name.to_owned(),
            self.name.clone(),
            ctx.address().recipient(),
        );

        Server::from_registry()
            .send(join_message)
            .into_actor(self)
            .then(|id, act, _ctx| {
                if let Ok(id) = id {
                    act.id = id;
                    act.room = room_name;
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn list_rooms(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        Server::from_registry()
            .send(ListRooms)
            .into_actor(self)
            .then(|res, _, ctx| {
                if let Ok(rooms) = res {
                    for room in rooms {
                        ctx.text(room);
                    }
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn send_message<T: Into<String>>(&self, message: T) {
        let message = message.into();
        let content = format!(
            "{}: {message}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
        );

        let message = SendMessage(self.room.clone(), self.id, content);

        // issue_async comes from having the `BrokerIssue` trait in scope.
        self.issue_system_async(message);
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.join_room("main", ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!(
            "Session closed for {}({}) in room {}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
            self.id,
            self.room
        );
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, message: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let message = match message {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(message) => message,
        };

        println!("WEBSOCKET MESSAGE: {message:?}");

        match message {
            ws::Message::Text(text) => {
                let message = text.trim();

                if message.starts_with('/') {
                    let mut command = message.splitn(2, ' ');

                    match command.next() {
                        Some("/list") => self.list_rooms(ctx),

                        Some("/join") => {
                            if let Some(room_name) = command.next() {
                                self.join_room(room_name, ctx);
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }

                        Some("/name") => {
                            if let Some(name) = command.next() {
                                self.name = Some(name.to_owned());
                                ctx.text(format!("name changed to: {name}"));
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }

                        _ => ctx.text(format!("!!! unknown command: {message:?}")),
                    }

                    return;
                }
                self.send_message(message);
            }
            ws::Message::Close(reason) => {
                let _ = ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}