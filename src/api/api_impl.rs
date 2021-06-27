use mizer_runtime::{RuntimeAccess, NodeDescriptor, DefaultRuntime};
use crate::{ApiCommand, ApiHandler};
use mizer_api::RuntimeApi;
use mizer_node::{NodeLink, NodeType, NodeDesigner, NodePath, PortId};
use mizer_layouts::Layout;
use mizer_nodes::{FixtureNode, Node};
use mizer_clock::{ClockState, ClockSnapshot};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Api {
    access: RuntimeAccess,
    sender: flume::Sender<ApiCommand>,
}

impl RuntimeApi for Api {
    fn nodes(&self) -> Vec<NodeDescriptor> {
        let designer = self.access.designer.read();
        self.access.nodes
            .iter()
            .map(|entry| entry.key().clone())
            .map(|path| self.get_descriptor(path, &designer))
            .collect()
    }

    fn links(&self) -> Vec<NodeLink> {
        self.access.links.read()
    }

    fn layouts(&self) -> Vec<Layout> {
        self.access.layouts.read()
    }

    fn add_layout(&self, name: String) {
        let mut layouts = self.access.layouts.read();
        layouts.push(Layout {
            id: name,
            controls: Default::default(),
        });
        self.access.layouts.set(layouts);
    }

    fn remove_layout(&self, id: String) {
        let mut layouts = self.access.layouts.read();
        layouts.retain(|layout| layout.id != id);
        self.access.layouts.set(layouts);
    }

    fn rename_layout(&self, id: String, name: String) {
        let mut layouts = self.access.layouts.read();
        if let Some(layout) = layouts.iter_mut().find(|layout| layout.id == id) {
            layout.id = name;
        }
        self.access.layouts.set(layouts);
    }

    fn add_node(
        &self,
        node_type: NodeType,
        designer: NodeDesigner,
    ) -> anyhow::Result<NodeDescriptor<'_>> {
        self.add_node_internal(node_type, designer, None)
    }

    fn add_node_for_fixture(&self, fixture_id: u32) -> anyhow::Result<NodeDescriptor<'_>> {
        let node = FixtureNode {
            fixture_id,
            ..Default::default()
        };
        self.add_node_internal(
            NodeType::Fixture,
            NodeDesigner::default(),
            Some(node.into()),
        )
    }

    fn write_node_port(&self, node_path: NodePath, port: PortId, value: f64) -> anyhow::Result<()> {
        let (tx, rx) = flume::bounded(1);
        self.sender
            .send(ApiCommand::WritePort(node_path, port, value, tx))?;
        let result = rx.recv()?;

        result
    }

    fn link_nodes(&self, link: NodeLink) -> anyhow::Result<()> {
        let (tx, rx) = flume::bounded(1);
        self.sender.send(ApiCommand::AddLink(link, tx))?;
        let result = rx.recv()?;

        result
    }

    fn get_node_history(&self, node: NodePath) -> anyhow::Result<Vec<f64>> {
        let (tx, rx) = flume::bounded(1);
        self.sender.send(ApiCommand::GetNodePreview(node, tx))?;
        let result = rx.recv()?;

        result
    }

    fn update_node(&self, path: NodePath, config: Node) -> anyhow::Result<()> {
        let (tx, rx) = flume::bounded(1);
        self.sender.send(ApiCommand::UpdateNode(path, config, tx))?;
        let result = rx.recv()?;

        result
    }

    fn set_clock_state(&self, state: ClockState) -> anyhow::Result<()> {
        self.sender.send(ApiCommand::SetClockState(state))?;

        Ok(())
    }

    fn save_project(&self) -> anyhow::Result<()> {
        let (tx, rx) = flume::bounded(1);
        self.sender.send(ApiCommand::SaveProject(tx))?;
        let result = rx.recv()?;

        result
    }

    fn transport_recv(&self) -> flume::Receiver<ClockSnapshot> {
        self.access.clock_recv.clone()
    }
}

impl Api {
    pub fn setup(runtime: &DefaultRuntime) -> (ApiHandler, Api) {
        let (tx, rx) = flume::unbounded();
        let access = runtime.access();

        (ApiHandler {
            recv: rx
        }, Api {
            sender: tx,
            access
        })
    }

    fn add_node_internal(
        &self,
        node_type: NodeType,
        designer: NodeDesigner,
        node: Option<Node>,
    ) -> anyhow::Result<NodeDescriptor<'_>> {
        let (tx, rx) = flume::bounded(1);
        self.sender
            .send(ApiCommand::AddNode(node_type, designer.clone(), node, tx))?;

        // TODO: this blocks, we should use the async method
        let path = rx.recv()??;
        let node = self.access.nodes.get(&path).unwrap();
        let ports = node.list_ports();

        Ok(NodeDescriptor {
            path,
            designer,
            node,
            ports,
        })
    }

    fn get_descriptor(
        &self,
        path: NodePath,
        designer: &HashMap<NodePath, NodeDesigner>,
    ) -> NodeDescriptor {
        let node = self.access.nodes.get(&path).unwrap();
        let ports = node.list_ports();
        let designer = designer[&path].clone();

        NodeDescriptor {
            path,
            node,
            designer,
            ports,
        }
    }
}