use mizer_node::{NodeContext, NodeDetails, NodeType, PipelineNode, ProcessingNode};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct TestSink {
    frames: Arc<Mutex<Vec<f64>>>,
}

impl PipelineNode for TestSink {
    fn details(&self) -> NodeDetails {
        NodeDetails {
            name: "TestSink".into(),
        }
    }

    fn node_type(&self) -> NodeType {
        unimplemented!()
    }
}

impl ProcessingNode for TestSink {
    type State = ();

    fn process(&self, context: &impl NodeContext, _: &mut Self::State) -> anyhow::Result<()> {
        if let Some(value) = context.read_port("input") {
            let mut frames = self.frames.lock().unwrap();
            frames.push(value);
        } else {
            println!("got no port");
        }

        Ok(())
    }

    fn create_state(&self) -> Self::State {
        ()
    }
}

impl TestSink {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn frames(&self) -> Vec<f64> {
        self.frames.lock().unwrap().clone()
    }
}