use actix::prelude::*;

pub use super::ChatMessage;

pub type Client = Recipient<ChatMessage>;