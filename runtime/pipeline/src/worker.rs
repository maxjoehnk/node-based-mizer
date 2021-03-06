use std::any::Any;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;

use downcast::*;
use ringbuffer::RingBufferExt;

use mizer_clock::ClockFrame;
use mizer_node::*;
use mizer_ports::memory::MemorySender;
use mizer_ports::PortValue;
use mizer_processing::Injector;
use mizer_protocol_laser::LaserFrame;

use crate::ports::{NodeReceivers, NodeSenders};
use crate::{NodePreviewState, PipelineContext};

pub trait ProcessingNodeExt: PipelineNode {
    fn process(&self, context: &PipelineContext, state: &mut Box<dyn Any>) -> anyhow::Result<()>;

    fn as_pipeline_node_mut(&mut self) -> &mut dyn PipelineNode;
}

downcast!(dyn ProcessingNodeExt);

impl<T, S: 'static> ProcessingNodeExt for T
where
    T: ProcessingNode<State = S>,
{
    fn process(&self, context: &PipelineContext, state: &mut Box<dyn Any>) -> anyhow::Result<()> {
        if let Some(state) = state.downcast_mut() {
            self.process(context, state)
        } else {
            unreachable!()
        }
    }

    fn as_pipeline_node_mut(&mut self) -> &mut dyn PipelineNode {
        self
    }
}

#[derive(Default)]
pub struct PipelineWorker {
    states: HashMap<NodePath, Box<dyn Any>>,
    senders: HashMap<NodePath, NodeSenders>,
    receivers: HashMap<NodePath, NodeReceivers>,
    dependencies: HashMap<NodePath, Vec<NodePath>>,
    previews: HashMap<NodePath, NodePreviewState>,
}

impl PipelineWorker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_node<T: 'static + ProcessingNode<State = S>, S: 'static>(
        &mut self,
        path: NodePath,
        node: &T,
    ) {
        let state = node.create_state();
        match mizer_node::ProcessingNode::details(node).preview_type {
            PreviewType::History => {
                self.previews
                    .insert(path.clone(), NodePreviewState::History(Default::default()));
            }
            _ => {
                self.previews.insert(path.clone(), NodePreviewState::None);
            }
        }
        let state: Box<S> = Box::new(state);
        self.states.insert(path.clone(), state);
        let mut receivers = NodeReceivers::default();
        for (port_id, metadata) in node.list_ports() {
            if metadata.is_input() {
                log::debug!("Registering port receiver for {:?} {:?}", &path, &port_id);
                match metadata.port_type {
                    PortType::Single => receivers.register::<f64>(port_id, metadata),
                    PortType::Multi => receivers.register::<Vec<f64>>(port_id, metadata),
                    PortType::Laser => receivers.register::<Vec<LaserFrame>>(port_id, metadata),
                    _ => {}
                }
            }
        }
        self.receivers.insert(path, receivers);
    }

    pub fn connect_nodes(
        &mut self,
        link: NodeLink,
        source_meta: PortMetadata,
        target_meta: PortMetadata,
    ) -> anyhow::Result<()> {
        self.add_dependency(link.source.clone(), link.target.clone());
        self.connect_ports(link, source_meta, target_meta)?;

        Ok(())
    }

    fn add_dependency(&mut self, node: NodePath, dependency: NodePath) {
        if let Some(dependencies) = self.dependencies.get_mut(&node) {
            dependencies.push(dependency);
        } else {
            self.dependencies.insert(node, vec![dependency]);
        }
    }

    fn connect_ports(
        &mut self,
        link: NodeLink,
        source_meta: PortMetadata,
        target_meta: PortMetadata,
    ) -> anyhow::Result<()> {
        match link.port_type {
            PortType::Single => self.connect_memory_ports::<f64>(link, source_meta, target_meta),
            PortType::Color => self.connect_memory_ports::<Color>(link, source_meta, target_meta),
            PortType::Multi => {
                self.connect_memory_ports::<Vec<f64>>(link, source_meta, target_meta)
            }
            PortType::Laser => {
                self.connect_memory_ports::<Vec<LaserFrame>>(link, source_meta, target_meta)
            }
            PortType::Gstreamer => self.connect_gst_ports(link)?,
            _ => unimplemented!(),
        }
        Ok(())
    }

    fn connect_memory_ports<V: PortValue + Default + 'static>(
        &mut self,
        link: NodeLink,
        source_meta: PortMetadata,
        target_meta: PortMetadata,
    ) {
        let senders = self
            .senders
            .entry(link.source)
            .or_insert(NodeSenders::default());
        let rx = if let Some((port, _)) = senders.get(link.source_port.clone()) {
            let port = port
                .downcast_ref::<MemorySender<V>>()
                .expect("port is not a memory port");

            // TODO: this may cause problems with ports reading the target metadata
            port.add_destination()
        } else {
            let (tx, rx) = mizer_ports::memory::channel::<V>();
            senders.add(link.source_port, tx, target_meta);

            rx
        };
        let receivers = self
            .receivers
            .entry(link.target)
            .or_insert(NodeReceivers::default());
        receivers.add(link.target_port, rx, source_meta);
    }

    fn connect_gst_ports(&self, link: NodeLink) -> anyhow::Result<()> {
        let source = self.states.get(&link.source).expect("invalid source path");
        let target = self.states.get(&link.target).expect("invalid target path");

        let gst_source = self.get_gst_node(source);
        let gst_target = self.get_gst_node(target);

        gst_source.link_to(gst_target)?;

        Ok(())
    }

    fn get_gst_node<'a>(&self, node: &'a Box<dyn Any>) -> &'a dyn mizer_video_nodes::GstreamerNode {
        use mizer_video_nodes::*;

        if let Some(node) = node.downcast_ref::<VideoColorBalanceState>() {
            node as &dyn GstreamerNode
        } else if let Some(node) = node.downcast_ref::<VideoOutputState>() {
            node as &dyn GstreamerNode
        } else if let Some(node) = node.downcast_ref::<VideoEffectState>() {
            node as &dyn GstreamerNode
        } else if let Some(node) = node.downcast_ref::<VideoTransformState>() {
            node as &dyn GstreamerNode
        } else if let Some(node) = node.downcast_ref::<VideoFileState>() {
            node as &dyn GstreamerNode
        } else {
            unreachable!()
        }
    }

    pub fn process<'a>(
        &mut self,
        mut nodes: Vec<(&'a NodePath, &'a Box<dyn ProcessingNodeExt>)>,
        frame: ClockFrame,
        injector: &Injector,
    ) {
        self.order_nodes_by_dependencies(&mut nodes);
        self.process_nodes(&mut nodes, frame, &injector)
    }

    fn order_nodes_by_dependencies(
        &mut self,
        nodes: &mut Vec<(&NodePath, &Box<dyn ProcessingNodeExt>)>,
    ) {
        nodes.sort_by(|(left, _), (right, _)| {
            let left_deps = self.dependencies.get(left).cloned().unwrap_or_default();
            let right_deps = self.dependencies.get(right).cloned().unwrap_or_default();

            if left_deps.contains(&right) {
                Ordering::Less
            } else if right_deps.contains(&left) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
    }

    fn process_nodes(
        &mut self,
        nodes: &mut Vec<(&NodePath, &Box<dyn ProcessingNodeExt>)>,
        frame: ClockFrame,
        injector: &Injector,
    ) {
        for (path, node) in nodes {
            let context = PipelineContext {
                frame,
                injector,
                preview: RefCell::new(self.previews.get_mut(path).unwrap()),
                receivers: self.receivers.get(path),
                senders: self.senders.get(path),
            };
            let state = self.states.get_mut(path);
            let state = state.unwrap();
            if let Err(e) = node.process(&context, state) {
                println!("processing of node failed: {:?}", e)
            }
        }
    }

    pub fn get_state<S: 'static>(&self, path: &NodePath) -> Option<&S> {
        self.states.get(path).and_then(|state| state.downcast_ref())
    }

    pub fn write_port<V: PortValue + 'static>(&mut self, path: NodePath, port: PortId, value: V) {
        if let Some(receivers) = self.receivers.get_mut(&path) {
            if let Some(receiver) = receivers.get(&port) {
                receiver.set_value(value);
            } else {
                log::warn!(
                    "trying to write value to unknown port {:?} on path {:?}",
                    &port,
                    &path
                );
            }
        } else {
            log::warn!(
                "trying to write value to unknown receiver for node {:?}",
                &path
            );
        }
    }

    pub fn get_history(&self, path: &NodePath) -> Option<Vec<f64>> {
        if let Some(preview) = self.previews.get(path) {
            match preview {
                NodePreviewState::History(vec) => Some(vec.to_vec()),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use mizer_node::{NodeLink, NodePath, PortDirection, PortId, PortMetadata, PortType};
    use mizer_ports::memory::MemorySender;
    use mizer_ports::NodePortSender;

    use crate::ports::NodeReceivers;

    use super::PipelineWorker;

    #[test_case(1.)]
    #[test_case(0.)]
    fn should_connect_two_nodes(value: f64) -> anyhow::Result<()> {
        let source_path: NodePath = "/source".into();
        let dest_path: NodePath = "/dest".into();
        let port_id: PortId = "value".into();
        let mut worker = PipelineWorker::new();
        let source = PortMetadata {
            port_type: PortType::Single,
            multiple: None,
            direction: PortDirection::Output,
            dimensions: None,
        };
        let dest = PortMetadata {
            port_type: PortType::Single,
            multiple: None,
            direction: PortDirection::Input,
            dimensions: None,
        };
        let link = NodeLink {
            port_type: PortType::Single,
            source_port: port_id.clone(),
            source: source_path.clone(),
            target: dest_path.clone(),
            target_port: port_id.clone(),
            local: true,
        };
        register_receiver(
            &mut worker,
            dest_path.clone(),
            port_id.clone(),
            dest.clone(),
        );

        worker.connect_nodes(link, source, dest)?;

        send_value(&worker, &source_path, port_id.clone(), value)?;
        let result = recv_value(&worker, &dest_path, &port_id);
        assert_eq!(value, result);
        Ok(())
    }

    #[test_case(1.)]
    #[test_case(0.)]
    fn should_write_from_one_output_to_multiple_inputs(value: f64) -> anyhow::Result<()> {
        let mut worker = PipelineWorker::new();
        let source = PortMetadata {
            port_type: PortType::Single,
            multiple: None,
            direction: PortDirection::Output,
            dimensions: None,
        };
        let dest = PortMetadata {
            port_type: PortType::Single,
            multiple: None,
            direction: PortDirection::Input,
            dimensions: None,
        };
        let source_path: NodePath = "/source".into();
        let dest1: NodePath = "/dest1".into();
        let dest2: NodePath = "/dest2".into();
        let port_id: PortId = "value".into();
        let link1 = NodeLink {
            port_type: PortType::Single,
            source_port: port_id.clone(),
            source: source_path.clone(),
            target: dest1.clone(),
            target_port: port_id.clone(),
            local: true,
        };
        let link2 = NodeLink {
            port_type: PortType::Single,
            source_port: port_id.clone(),
            source: source_path.clone(),
            target: dest2.clone(),
            target_port: port_id.clone(),
            local: true,
        };
        register_receiver(&mut worker, dest1.clone(), port_id.clone(), dest.clone());
        register_receiver(&mut worker, dest2.clone(), port_id.clone(), dest.clone());

        worker.connect_nodes(link1, source, dest)?;
        worker.connect_nodes(link2, source, dest)?;

        send_value(&worker, &source_path, port_id.clone(), value)?;
        let result1 = recv_value(&worker, &dest1, &port_id);
        let result2 = recv_value(&worker, &dest2, &port_id);
        assert_eq!(value, result1);
        assert_eq!(value, result2);
        Ok(())
    }

    fn register_receiver(
        worker: &mut PipelineWorker,
        path: NodePath,
        port: PortId,
        meta: PortMetadata,
    ) {
        let mut recvs = NodeReceivers::default();
        recvs.register::<f64>(port, meta);
        worker.receivers.insert(path, recvs);
    }

    fn send_value(
        worker: &PipelineWorker,
        path: &NodePath,
        port: PortId,
        value: f64,
    ) -> anyhow::Result<()> {
        let sender = worker.senders.get(path).unwrap();
        let (port, _) = sender.get(port).unwrap();
        let port = port.downcast_ref::<MemorySender<f64>>().unwrap();

        port.send(value)
    }

    fn recv_value(worker: &PipelineWorker, path: &NodePath, port: &PortId) -> f64 {
        let recv = worker.receivers.get(path).unwrap();
        let port = recv.get(port).unwrap();

        port.read().unwrap()
    }
}
