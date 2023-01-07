use actix_web_lab::sse as awl_sse;
use std::collections::HashMap;

/// Struct container for message
#[derive(Debug, Clone)]
pub struct BroadcasterInner {
    pub clients: HashMap<String, Vec<awl_sse::Sender>>,
}