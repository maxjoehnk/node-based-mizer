use mizer_clock::ClockFrame;
use mizer_node::*;
use mizer_ports::memory::{MemoryReceiver, MemorySender};
use mizer_ports::{NodePortReceiver, NodePortSender, PortId, PortValue};
use mizer_processing::Injector;

use crate::ports::{NodeReceivers, NodeSenders};

pub struct PipelineContext<'a> {
    pub(crate) frame: ClockFrame,
    pub(crate) senders: Option<&'a NodeSenders>,
    pub(crate) receivers: Option<&'a NodeReceivers>,
    pub(crate) injector: &'a Injector,
}

impl<'a> NodeContext for PipelineContext<'a> {
    fn clock(&self) -> ClockFrame {
        self.frame
    }

    fn write_port<P: Into<PortId>, V: PortValue + 'static>(&self, port: P, value: V) {
        let port = port.into();
        let dbg_msg = format!("Trying to write to non existent port {}", &port);
        if let Some((port, _)) = self.senders.and_then(|senders| senders.get(port)) {
            let port = port
                .downcast_ref::<MemorySender<V>>()
                .expect("can't downcast sender to proper type");
            if let Err(e) = port.send(value) {
                log::error!("Sending data via port failed: {:?}", e);
            }
        } else {
            log::debug!("{}", dbg_msg);
        }
    }

    fn read_port<P: Into<PortId>, V: PortValue + 'static>(&self, port: P) -> Option<V> {
        let port = port.into();
        self.receivers
            .and_then(|receivers| receivers.get(&port))
            .and_then(|receiver| receiver.read())
    }

    // TODO: return as ref again?
    fn input_port<P: Into<PortId>>(&self, port: P) -> PortMetadata {
        let port = port.into();
        self.receivers
            .and_then(|ports| ports.get(&port))
            .map(|recv| recv.metadata.clone())
            .unwrap()
    }

    fn output_port<P: Into<PortId>>(&self, port: P) -> &PortMetadata {
        let port = port.into();
        let (_, metadata) = self.senders.and_then(|ports| ports.get(port)).unwrap();

        metadata
    }

    fn input_ports(&self) -> Vec<PortId> {
        self.receivers
            .map(|receivers| receivers.ports())
            .unwrap_or_default()
    }

    fn inject<T: 'static>(&self) -> Option<&T> {
        self.injector.get::<T>()
    }
}
