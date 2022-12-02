use actix_web::rt::time::interval;
use actix_web_lab::sse as awl_sse;
use futures_util::future;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};

/// Static for subscription ping
pub static SUBSCRIPTION_PING: &'static str = "ping";

/// Static default connection action
pub static SUBSCRIPTION_CONNECTION_ACTION: &'static str = "connection";

/// Static content
pub static SUBSCRIPTION_CONNECTION_CONTENT: &'static str = "Successfully Connected";

/// Static module
pub static SUBSCRIPTION_CONNECTION_MODULE: &'static str = "SSE";

/// Static event
pub static SUBSCRIPTION_EVENT: &'static str = "message";

/// Create broadcaster struct
pub struct SSEBroadcaster {
    inner: Mutex<SSEBroadcasterInner>,
}

/// Create inner broadcaster struct
#[derive(Debug, Clone)]
pub struct SSEBroadcasterInner {
    clients: HashMap<String, Vec<awl_sse::Sender>>,
}

// Implement broadcaster functions
impl SSEBroadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn new() -> Arc<Self> {
        // Create broadcaster with channel
        let this = Arc::new(SSEBroadcaster {
            inner: Mutex::new(SSEBroadcasterInner {
                clients: HashMap::<String, Vec<awl_sse::Sender>>::new()
            }),
        });

        // Spawn ping in a loop so it won't drop the stream right away
        SSEBroadcaster::spawn_ping(Arc::clone(&this));

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
                if sender.send(awl_sse::Event::Comment(SUBSCRIPTION_PING.into())).await.is_ok() {
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
        let mut message = SSEData::default();
        message.action = Some(String::from(SUBSCRIPTION_CONNECTION_ACTION));
        message.content = Some(String::from(SUBSCRIPTION_CONNECTION_CONTENT));
        message.module = Some(String::from(SUBSCRIPTION_CONNECTION_MODULE));

        // Create data
        let content = r#"{"action": "connection", "content": "Successfully Connected", "module": "SSE"}"#;
        let data = serde_json::to_string(&message).unwrap_or(String::from(content));

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
    pub async fn broadcast(&self, params: &SSEMessage) {
        // Set channel, data and event
        let mut channel = String::from("Global");
        let mut data = String::new();
        let mut event = String::from(SUBSCRIPTION_EVENT);

        // Set channel
        if params.channel.is_some() && !params.channel.as_ref().unwrap().is_empty() {
            channel = params.channel.as_ref().unwrap().clone();
        }

        // Set message data
        if params.data.is_some() {
            let result = serde_json::to_string(&params.data.as_ref().unwrap());
            if result.is_ok() {
                let content = String::from(r#"{"action": "error", "content": "Failed to send data", "module": "error"}"#);
                data = result.unwrap_or(content);
            }
        }

        // Set event
        if params.event.is_some() && !params.event.as_ref().unwrap().is_empty() {
            event = params.event.as_ref().unwrap().clone();
        }

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

/// Create SSEData struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSEData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
}

// Implement default for SSEData
impl Default for SSEData {
    fn default() -> Self {
        Self {
            action: None,
            content: None,
            module: None
        }
    }
}

// Create SSEData implementation
impl SSEData {
    /// Create new instance
    ///
    /// Example
    /// ```
    /// use library::sse::SSEData;
    ///
    /// let data = SSEData::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

/// Create SSEMessage struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSEMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SSEData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

// Implement default for SSEMessage
impl Default for SSEMessage {
    fn default() -> Self {
        Self {
            channel: None,
            data: None,
            event: None
        }
    }
}

// Create SSEMessage implementation
impl SSEMessage {
    /// Create new instance
    ///
    /// Example
    /// ```
    /// use library::sse::SSEMessage;
    ///
    /// let message = SSEMessage::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}