use mizer_protocol_dmx::DmxOutput;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Fixture {
    pub id: u32,
    pub definition: FixtureDefinition,
    pub current_mode: FixtureMode,
    pub universe: u16,
    pub channel: u8,
    pub output: String,
    pub channel_values: HashMap<String, f64>,
}

impl Fixture {
    pub fn new(
        fixture_id: u32,
        definition: FixtureDefinition,
        selected_mode: Option<String>,
        output: String,
        channel: u8,
        universe: Option<u16>,
    ) -> Self {
        Fixture {
            id: fixture_id,
            current_mode: get_current_mode(&definition, selected_mode),
            definition,
            channel,
            output,
            universe: universe.unwrap_or(1),
            channel_values: Default::default(),
        }
    }

    pub fn get_channels(&self) -> Vec<FixtureChannelDefinition> {
        self.current_mode.channels.clone()
    }

    pub fn write(&mut self, name: &str, value: f64) {
        log::debug!("write {} -> {}", name, value);
        self.channel_values.insert(name.to_string(), value);
    }

    pub(crate) fn flush(&self, output: &dyn DmxOutput) {
        let buffer = self.get_dmx_values();
        let start = self.channel as usize;
        let end = start + self.current_mode.dmx_channels() as usize;
        output.write_bulk(self.universe, self.channel, &buffer[start..end]);
    }

    pub fn get_dmx_values(&self) -> [u8; 255] {
        let mut buffer = [0; 255];

        for (channel_name, value) in self.channel_values.iter() {
            if let Some(channel) = self
                .current_mode
                .channels
                .iter()
                .find(|channel| &channel.name == channel_name)
            {
                match channel.resolution {
                    ChannelResolution::Coarse(coarse) => {
                        let channel = (self.channel + coarse) as usize;
                        buffer[channel] = convert_value(*value);
                    }
                    _ => unimplemented!("only coarse is implemented right now"),
                }
            }
        }

        buffer
    }
}

fn convert_value(input: f64) -> u8 {
    let clamped = input.min(1.0).max(0.0);
    let channel = clamped * (u8::MAX as f64);

    channel.floor() as u8
}

fn get_current_mode(definition: &FixtureDefinition, selected_mode: Option<String>) -> FixtureMode {
    if let Some(selected_mode) = selected_mode {
        definition
            .modes
            .iter()
            .find(|mode| mode.name == selected_mode)
            .cloned()
            .expect("invalid fixture mode")
    } else {
        definition.modes[0].clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureDefinition {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub modes: Vec<FixtureMode>,
    pub physical: PhysicalFixtureData,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureMode {
    pub name: String,
    pub channels: Vec<FixtureChannelDefinition>,
    pub groups: Vec<FixtureChannelGroup>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureChannelGroup {
    pub name: String,
    pub group_type: FixtureChannelGroupType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FixtureChannelGroupType {
    Generic(String),
    Color(ColorGroup),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorGroup {
    pub red: String,
    pub green: String,
    pub blue: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Angle {
    pub from: f32,
    pub to: f32,
}

impl FixtureMode {
    fn dmx_channels(&self) -> u8 {
        self.channels.iter().map(|c| c.channels()).sum()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureChannelDefinition {
    pub name: String,
    pub resolution: ChannelResolution,
}

impl FixtureChannelDefinition {
    fn channels(&self) -> u8 {
        match self.resolution {
            ChannelResolution::Coarse { .. } => 1,
            ChannelResolution::Fine { .. } => 2,
            ChannelResolution::Finest { .. } => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelResolution {
    /// 8 Bit
    ///
    /// coarse
    Coarse(u8),
    /// 16 Bit
    ///
    /// coarse, fine
    Fine(u8, u8),
    /// 24 Bit
    ///
    /// coarse, fine, finest
    Finest(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicalFixtureData {
    pub dimensions: Option<FixtureDimensions>,
    pub weight: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixtureDimensions {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
