mod user_agent_cpu;
mod user_agent_device;
mod user_agent_engine;
mod user_agent_os;
mod user_agent_product;

use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, web::Data};
use actix_web::dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::{ok, Ready};
use serde::Serialize;
use user_agent_parser::UserAgentParser as UAParser;

use user_agent_cpu::UserAgentCPU;
use user_agent_device::UserAgentDevice;
use user_agent_engine::UserAgentEngine;
use user_agent_os::UserAgentOS;
use user_agent_product::UserAgentProduct;

// Create user agent parser
pub struct UserAgentParser;

// Create implementation for user agent parser
impl UserAgentParser {
    pub fn new() -> Self {
        Self
    }
}

// Implement default for user agent parser
impl Default for UserAgentParser {
    fn default() -> Self {
        Self::new()
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for UserAgentParser
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = UserAgentParserMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(UserAgentParserMiddleware { service })
    }
}

// Create user agent parser middleware service struct
pub struct UserAgentParserMiddleware<S> {
    service: S,
}

// Implement service for middleware
impl<S, B> Service<ServiceRequest> for UserAgentParserMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Retrieve user agent
        let ua_str = req
            .headers()
            .get("User-Agent")
            .map(|h| h.to_str().unwrap_or(""))
            .unwrap_or("");

        // Retrieve request
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .map_or(String::default(), |item| item.to_string());

        // Retrieve parser
        let parser = req.app_data::<Data<UAParser>>()
            .unwrap()
            .clone();

        // Insert user agent object
        req.extensions_mut().insert(UserAgent::from_parser(&parser, ua_str, &ip));

        // Return service call req
        self.service.call(req)
    }
}


// Create user agent object
#[derive(Debug, Clone, Serialize)]
pub struct UserAgent {
    pub ip: Option<String>,
    pub product: UserAgentProduct,
    pub os: UserAgentOS,
    pub device: UserAgentDevice,
    pub cpu: UserAgentCPU,
    pub engine: UserAgentEngine,
}

// Create implementation for UserAgent
impl UserAgent {
    // Creates a new user agent
    pub fn new() -> Self {
        Self {
            ip: None,
            product: UserAgentProduct::new(),
            os: UserAgentOS::new(),
            device: UserAgentDevice::new(),
            cpu: UserAgentCPU::new(),
            engine: UserAgentEngine::new(),
        }
    }

    // Creates user agent from parsed user agent string
    pub fn from_parser<T: Into<String>>(parser: &UAParser, ua_str: &str, ip: T) -> Self {
        // Set bindings
        let bindings = ip.into();

        let ua_product =  parser.parse_product(ua_str);
        let ua_os = parser.parse_os(ua_str);
        let ua_device = parser.parse_device(ua_str);
        let ua_cpu = parser.parse_cpu(ua_str);
        let ua_engine = parser.parse_engine(ua_str);

        // Create user agent
        let mut user_agent = UserAgent::new();

        // Set product
        user_agent.product.name = ua_product.name.and_then(|item| { Some(item.to_string()) });
        user_agent.product.major = ua_product.major.and_then(|item| { Some(item.to_string()) });
        user_agent.product.minor = ua_product.minor.and_then(|item| { Some(item.to_string()) });
        user_agent.product.patch = ua_product.patch.and_then(|item| { Some(item.to_string()) });

        // Set os
        user_agent.os.name = ua_os.name.and_then(|item| { Some(item.to_string()) });
        user_agent.os.major = ua_os.major.and_then(|item| { Some(item.to_string()) });
        user_agent.os.minor = ua_os.minor.and_then(|item| { Some(item.to_string()) });
        user_agent.os.patch = ua_os.patch.and_then(|item| { Some(item.to_string()) });
        user_agent.os.patch_minor = ua_os.patch_minor.and_then(|item| { Some(item.to_string()) });

        // Set device
        user_agent.device.name = ua_device.name.and_then(|item| { Some(item.to_string()) });
        user_agent.device.brand = ua_device.brand.and_then(|item| { Some(item.to_string()) });
        user_agent.device.model = ua_device.model.and_then(|item| { Some(item.to_string()) });

        // Set architecture
        user_agent.cpu.architecture = ua_cpu.architecture.and_then(|item| { Some(item.to_string()) });

        // Set engine
        user_agent.engine.name = ua_engine.name.and_then(|item| { Some(item.to_string()) });
        user_agent.engine.major = ua_engine.major.and_then(|item| { Some(item.to_string()) });
        user_agent.engine.minor = ua_engine.minor.and_then(|item| { Some(item.to_string()) });
        user_agent.engine.patch = ua_engine.patch.and_then(|item| { Some(item.to_string()) });

        // Check if ip is not empty
        if !bindings.is_empty() {
            user_agent.ip = Some(bindings);
        }

        // Return user agent
        user_agent
    }

    // Convert self to json value
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self.clone()).unwrap()
    }
}

// Implement default for user agent
impl Default for UserAgent {
    fn default() -> Self {
        Self::new()
    }
}

// Implement from request
impl FromRequest for UserAgent {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        return match req.extensions().get::<UserAgent>() {
            Some(user_agent) => ok(user_agent.clone()),
            None => ok(UserAgent::new())
        };
    }
}

