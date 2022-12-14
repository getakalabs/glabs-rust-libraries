use actix_web::rt::time::interval;
use actix_web_lab::sse as awl_sse;
use futures_util::future;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};

use super::BroadcasterInner;
use super::Data;
use super::Message;
use super::System;

/// Create broadcaster struct
pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
    system: System,
}

// Implement broadcaster functions
impl Broadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn new(system: System) -> Arc<Self> {
        // Create system
        let mut sys = System::new();
        if !system.is_empty() {
            sys = system.clone();
        }

        // Create broadcaster with channel
        let this = Arc::new(Broadcaster {
            inner: Mutex::new(super::BroadcasterInner {
                clients: HashMap::<String, Vec<awl_sse::Sender>>::new()
            }),
            system: sys
        });

        // Return broadcaster
        this
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast list if not.
    pub fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    pub async fn remove_stale_clients(&self) {
        // Retrieve list of clients
        let clients = self.inner.lock().clients.clone();

        // Initialize clients that are still ok
        let mut ok_clients = HashMap::<String, Vec<awl_sse::Sender>>::new();

        // Loop through clients
        for (client, senders) in clients {
            // Create senders that are still ok
            let mut ok_senders:Vec<awl_sse::Sender> = Vec::new();

            // loop through all senders
            for sender in senders {
                // Send ping
                if sender.send(awl_sse::Event::Comment(self.system.clone().ping.into())).await.is_ok() {
                    ok_senders.push(sender.clone())
                }
            }

            // Check if senders has value
            if ok_senders.len() > 0 {
                ok_clients.insert(client, ok_senders);
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client<T: Into<String>>(&self, channel: T) -> awl_sse::Sse<awl_sse::ChannelStream> {
        // Initialize sender and stream tuple
        let (sender, stream) = awl_sse::channel(10);

        // Create message data
        let mut message = Data::default();
        message.action = Some(String::from(self.system.clone().action));
        message.content = Some(String::from(self.system.clone().content));
        message.module = Some(String::from(self.system.clone().module));

        // Create data
        let content = r#"{"action": "connection", "content": "Successfully Connected", "module": "SSE"}"#;
        let data = serde_json::to_string(&message)
            .unwrap_or(String::from(content));

        // Send connected message
        sender.send(awl_sse::Data::new(data)).await.unwrap();

        // Set clients
        let mut clients = self.inner.lock().clients.clone();

        // Insert client if it exists
        let bindings = channel.into();
        match clients.get(&bindings) {
            None => {
                // Create channel vector
                let mut ch = Vec::new();
                ch.push(sender);

                clients.insert(bindings.clone(), ch);
                self.inner.lock().clients = clients;
            },
            Some(client) => {
                let mut c = client.clone();
                c.push(sender);
                clients.insert(bindings.clone(), c);
                self.inner.lock().clients = clients;
            }
        }

        stream
    }

    /// Broadcasts message to all clients within the channel.
    pub async fn broadcast(&self, params: &Message) {
        // Set channel, data and event
        let mut data = String::new();

        // Set global channel string
        let default_channel = String::from("Global");

        // Set channel
        let channel = params.channel
            .clone()
            .map_or(default_channel.clone(), |s| {
                s.is_empty()
                    .then(|| default_channel.clone())
                    .unwrap_or(default_channel.clone())
            });

        // Set message data
        if params.data.is_some() {
            let result = serde_json::to_string(&params.data.as_ref().unwrap());
            if result.is_ok() {
                let content = String::from(r#"{"action": "error", "content": "Failed to send data", "module": "error"}"#);
                data = result.unwrap_or(content);
            }
        }

        // Set default event
        let default_event = String::from(self.system.clone().event);

        // Set event
        let event = params.event
            .clone()
            .map_or(default_event.clone(), |s| {
                s.is_empty()
                    .then(|| default_event.clone())
                    .unwrap_or(default_event.clone())
            });

        // Check if channel exists
        match self.inner.lock().clients.get(channel.as_str()) {
            Some(clients) => {
                let send_futures = clients
                    .iter()
                    .map(|client| client.send(awl_sse::Data::new(data.as_str()).event(event.as_str())));

                // try to send to all clients, ignoring failures
                // disconnected clients will get swept up by `remove_stale_clients`
                let _ = future::join_all(send_futures).await;
            },
            None => {}
        }
    }
}