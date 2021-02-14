use gstreamer::prelude::*;
use gstreamer::{DebugGraphDetails, Element, ElementFactory};

use mizer_node_api::*;

use crate::PIPELINE;

pub struct VideoOutputNode {
    sink: Element,
}

impl VideoOutputNode {
    pub fn new() -> Self {
        let pipeline = PIPELINE.lock().unwrap();
        let sink = ElementFactory::make("glimagesinkelement", None).unwrap();
        pipeline.add(&sink).unwrap();
        gstreamer::debug_bin_to_dot_file(&*pipeline, DebugGraphDetails::ALL, "output");

        VideoOutputNode { sink }
    }
}

impl Default for VideoOutputNode {
    fn default() -> Self {
        VideoOutputNode::new()
    }
}

impl ProcessingNode for VideoOutputNode {
    fn get_details(&self) -> NodeDetails {
        NodeDetails::new("VideoOutputNode")
            .with_inputs(vec![NodeInput::new("input", NodeChannel::Video)])
    }

    fn process(&mut self) {
        let pipeline = PIPELINE.lock().unwrap();
        pipeline.set_state(gstreamer::State::Playing).unwrap();
    }
}
impl SourceNode for VideoOutputNode {
    fn connect_video_input(&mut self, input: &str, element: &impl ElementExt) -> ConnectionResult {
        if input == "input" {
            element.link(&self.sink)?;
            let pipeline = PIPELINE.lock().unwrap();
            gstreamer::debug_bin_to_dot_file(&*pipeline, DebugGraphDetails::ALL, "linked");
            Ok(())
        } else {
            Err(ConnectionError::InvalidInput)
        }
    }
}
impl DestinationNode for VideoOutputNode {}