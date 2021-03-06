use std::any::Any;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use zeroconf::prelude::*;
use zeroconf::{MdnsBrowser, MdnsService, ServiceDiscovery, TxtRecord};

const MIZER_SESSION_SERVICE: &str = "_mizer._tcp";

const POLL_TIMEOUT: u64 = 1;

fn announce_device() {
    thread::Builder::new()
        .name("Session MDNS Broadcast".into())
        .spawn(|| {
            let mut service = MdnsService::new(MIZER_SESSION_SERVICE, 50051);
            let mut txt_record = TxtRecord::new();
            txt_record.insert("project", "video.yml").unwrap();
            service.set_txt_record(txt_record);
            let event_loop = service.register().unwrap();
            loop {
                event_loop.poll(Duration::from_secs(POLL_TIMEOUT)).unwrap();
                #[cfg(target_os = "linux")]
                // poll doesn't sleep in avahi implementation
                thread::sleep(Duration::from_secs(POLL_TIMEOUT));
            }
        })
        .unwrap();
}

pub fn discover_sessions() -> anyhow::Result<()> {
    let mut browser = MdnsBrowser::new(MIZER_SESSION_SERVICE);

    browser.set_service_discovered_callback(Box::new(on_service_discovered));

    let event_loop = browser.browse_services().unwrap();
    loop {
        event_loop.poll(Duration::from_secs(POLL_TIMEOUT)).unwrap();
    }
}

fn on_service_discovered(
    result: zeroconf::Result<ServiceDiscovery>,
    _context: Option<Arc<dyn Any>>,
) {
    println!("service: {:?}", result);
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub clients: Vec<SessionClient>,
}

impl Session {
    pub fn new() -> anyhow::Result<Self> {
        announce_device();
        Ok(Session {
            clients: vec![SessionClient::get_self()?],
        })
    }

    // TODO: discover and connect to remote session
    pub fn discover() -> anyhow::Result<Self> {
        Ok(Session {
            clients: vec![SessionClient::get_self()?],
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionClient {
    pub name: String,
    pub ips: Vec<String>,
    pub orchestrator: bool,
}

impl SessionClient {
    pub fn get_self() -> anyhow::Result<Self> {
        let name = hostname::get()?.into_string().unwrap();

        Ok(SessionClient {
            name,
            ips: vec![],
            orchestrator: true,
        })
    }
}

#[allow(dead_code)]
enum ClientRole {
    Orchestrator, // single point of truth, only client which can directly modify session
    Replica,      // may take over the orchestrator role when it leaves the session
    Worker,       // raw number cruncher
    Edge,         // outputs video, controls dmx universe etc
}
