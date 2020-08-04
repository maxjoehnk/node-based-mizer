use crate::nodes::Node;
use mizer_node_api::*;
use mizer_project_files::{Project, NodeConfig};
use std::collections::HashMap;
use crate::nodes::*;
use anyhow::{anyhow, Context};
use multi_mut::HashMapMultiMut;

#[derive(Debug, Default)]
pub struct Pipeline<'a> {
    nodes: HashMap<String, Node<'a>>,
}

impl<'a> Pipeline<'a> {
    pub fn load_project(&mut self, project: Project) -> anyhow::Result<()> {
        for node in project.nodes {
            let id = node.id.clone();
            let node = node.build();
            self.nodes.insert(id, node);
        }

        for channel in project.channels {
            let (lhs, rhs) = self.nodes.get_pair_mut(&channel.from_id, &channel.to_id)
                .ok_or_else(|| anyhow!("unknown node {} or {}", &channel.from_id, &channel.to_id))?;
            let context = format!("{:?}\noutput: {:?}\ninput: {:?}", &channel, lhs, rhs);
            match lhs.get_details().get_output_type(&channel.from_channel) {
                Some(NodeChannel::Dmx) => lhs.connect_to_dmx_input(&channel.from_channel, rhs, &channel.to_channel).context(context)?,
                Some(NodeChannel::Numeric) => lhs.connect_to_numeric_input(&channel.from_channel, rhs, &channel.to_channel).context(context)?,
                Some(NodeChannel::Trigger) => lhs.connect_to_trigger_input(&channel.from_channel, rhs, &channel.to_channel).context(context)?,
                Some(NodeChannel::Clock) => lhs.connect_to_clock_input(&channel.from_channel, rhs, &channel.to_channel).context(context)?,
                Some(NodeChannel::Video) => lhs.connect_to_video_input(&channel.from_channel, rhs, &channel.to_channel).context(context)?,
                Some(NodeChannel::Pixels) => lhs.connect_to_pixel_input(&channel.from_channel, rhs, &channel.to_channel).context(context)?,
                Some(channel) => unimplemented!("channel not implemented {:?}", channel),
                None => return Err(anyhow!("unknown output").context(context))
            }
        }

        Ok(())
    }

    pub fn process(&mut self) {
        for (id, node) in self.nodes.iter_mut() {
            let before = std::time::Instant::now();
            node.process();
            let after = std::time::Instant::now();
            metrics::timing!("mizer.process_time", before, after, "id" => id.clone());
        }
    }
}

trait NodeBuilder {
    fn build<'a>(self) -> Node<'a>;
}

impl NodeBuilder for mizer_project_files::Node {
    fn build<'a>(self) -> Node<'a> {
        let mut node: Node = match self.config {
            NodeConfig::ArtnetOutput { host, port } => ArtnetOutputNode::new(host, port).into(),
            NodeConfig::PixelPattern { pattern } => PixelPatternGeneratorNode::new(pattern).into(),
            NodeConfig::OpcOutput { host, port, width, height } => OpcOutputNode::new(host, port, (width, height)).into(),
            NodeConfig::ConvertToDmx { universe, channel } => ConvertToDmxNode::new(universe, channel).into(),
            NodeConfig::Oscillator { oscillator_type } => OscillatorNode::new(oscillator_type).into(),
            NodeConfig::Clock { speed } => ClockNode::new(speed).into(),
            NodeConfig::Script(script) => ScriptingNode::new(script.as_str()).into(),
            NodeConfig::OscInput { host, port, path } => OscInputNode::new(host, port, path).into(),
            NodeConfig::Fader => FaderNode::new().into(),
            NodeConfig::VideoFile { file } => VideoFileNode::new(file).into(),
            NodeConfig::VideoEffect { effect_type } => VideoEffectNode::new(effect_type).into(),
            NodeConfig::VideoTransform => VideoTransformNode::new().into(),
            NodeConfig::VideoOutput => VideoOutputNode::new().into(),
        };
        for (key, value) in self.properties {
            node.set_numeric_property(&key, value);
        }
        node
    }
}
