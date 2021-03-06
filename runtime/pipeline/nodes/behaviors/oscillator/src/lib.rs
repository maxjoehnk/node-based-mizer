use std::f64::consts::PI;

use serde::{Deserialize, Serialize};

use mizer_node::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OscillatorType {
    Square,
    Sine,
    Saw,
    Triangle,
}

impl Default for OscillatorType {
    fn default() -> Self {
        OscillatorType::Sine
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct OscillatorNode {
    #[serde(rename = "type")]
    pub oscillator_type: OscillatorType,
    #[serde(default = "default_ratio")]
    pub ratio: f64,
    #[serde(default = "default_max")]
    pub max: f64,
    #[serde(default = "default_min")]
    pub min: f64,
    #[serde(default)]
    pub offset: f64,
    #[serde(default)]
    pub reverse: bool,
}

impl Default for OscillatorNode {
    fn default() -> Self {
        OscillatorNode {
            oscillator_type: Default::default(),
            ratio: 1f64,
            max: 1f64,
            min: 0f64,
            offset: 0f64,
            reverse: false,
        }
    }
}

fn default_ratio() -> f64 {
    OscillatorNode::default().ratio
}

fn default_min() -> f64 {
    OscillatorNode::default().min
}

fn default_max() -> f64 {
    OscillatorNode::default().max
}

impl PipelineNode for OscillatorNode {
    fn details(&self) -> NodeDetails {
        NodeDetails {
            name: "OscillatorNode".into(),
            preview_type: PreviewType::History,
        }
    }

    fn node_type(&self) -> NodeType {
        NodeType::Oscillator
    }

    fn introspect_port(&self, port: &PortId) -> Option<PortMetadata> {
        (port == "value").then(|| PortMetadata {
            port_type: PortType::Single,
            direction: PortDirection::Output,
            ..Default::default()
        })
    }

    fn list_ports(&self) -> Vec<(PortId, PortMetadata)> {
        vec![(
            "value".into(),
            PortMetadata {
                port_type: PortType::Single,
                direction: PortDirection::Output,
                ..Default::default()
            },
        )]
    }
}

impl ProcessingNode for OscillatorNode {
    type State = ();

    fn process(&self, context: &impl NodeContext, _state: &mut Self::State) -> anyhow::Result<()> {
        let clock = context.clock();
        let value = self.tick(clock.frame);
        context.write_port("value", value);
        context.push_history_value(value);
        Ok(())
    }

    fn create_state(&self) -> Self::State {
        Default::default()
    }
}

impl OscillatorNode {
    fn tick(&self, beat: f64) -> f64 {
        match &self.oscillator_type {
            OscillatorType::Square => {
                let base = self.ratio * 0.5;
                let frame = self.get_frame(beat);
                if frame > base {
                    self.min
                } else {
                    self.max
                }
            }
            OscillatorType::Sine => {
                let min = self.min;
                let max = self.max;
                let offset = (max - min) / 2f64;
                let value = f64::sin(
                    (3f64 / 2f64) * PI + PI * ((beat + self.offset) * 2f64) * (1f64 / self.ratio),
                ) * offset
                    + offset
                    + min;
                let value = value.max(min).min(max);
                log::trace!(
                    "min: {}, max: {}, offset: {}, result: {}",
                    min,
                    max,
                    offset,
                    value
                );
                value
            }
            OscillatorType::Triangle => {
                let base = self.ratio / 2f64;
                let frame = self.get_frame(beat);
                let time = if frame > base { frame - base } else { frame };
                let high = self.max;
                let low = self.min;
                let value = ((high - low) / self.ratio) * time + low;
                if frame > base {
                    high - value
                } else {
                    value
                }
            }
            oscillator => unimplemented!("{:?}", oscillator),
        }
    }

    fn get_frame(&self, beat: f64) -> f64 {
        let mut frame = beat + self.offset;
        while frame > self.ratio {
            frame -= self.ratio;
        }
        frame
    }
}
