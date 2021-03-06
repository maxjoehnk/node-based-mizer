use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Deref;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct PortId(pub String);

impl Display for PortId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<String> for PortId {
    fn from(id: String) -> Self {
        PortId(id)
    }
}

impl From<&str> for PortId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl PartialEq<&str> for &PortId {
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<String> for &PortId {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PortId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum PortType {
    /// Single float value
    ///
    /// e.g. Control signals
    Single,
    /// Multiple float values
    ///
    /// e.g. Audio
    Multi,
    // TODO: should this actually be a texture?
    /// RGBA Colors
    Color,
    /// 2D Image and Video
    Texture,
    // TODO: maybe merge Vector and Poly?
    /// Vector Data
    ///
    /// e.g. SVG
    Vector,
    // TODO: maybe merge into vector?
    /// Laser Frames
    Laser,
    /// 3D Data
    ///
    /// e.g. 3D Objects
    Poly,
    /// Text Data
    ///
    /// e.g. Scripts
    Data,
    // TODO: figure out what kind's of data should be transmitted here
    /// Materials and Shaders for 3D Objects
    Material,
    /// Legacy, should be converted to texture
    Gstreamer,
}

impl Default for PortType {
    fn default() -> Self {
        PortType::Single
    }
}

pub trait NodePortReceiver<'a, Item>
where
    Item: PortValue,
{
    type Guard: ReceiverGuard<Item>;

    fn recv(&'a self) -> Option<Self::Guard>;
}

pub trait ReceiverGuard<Item>: Deref<Target = Item>
where
    Item: PortValue,
{
}

pub trait NodePortSender<Item>
where
    Item: PortValue,
{
    fn send(&self, value: Item) -> anyhow::Result<()>;
}

pub trait PortValue: Debug + Clone + PartialEq + Send + Sync + Any {}

impl<T> PortValue for T where T: Debug + Clone + PartialEq + Send + Sync + Any {}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
