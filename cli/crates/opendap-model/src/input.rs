//! Input events sent to a device (`opendap.input/v1`).

use serde::{Deserialize, Serialize};

use crate::device::Transport;

/// What a control means, and the events a host sends a device: the vocabulary and the wire the
/// device and the host share (system `opendap-bus-model`, RFC 0020).
pub use opendap_bus_model::{InputEvent, Role, UnknownRole};

/// An event that reached the device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputSent {
    pub target: Transport,
    pub event: InputEvent,
    pub changed: bool,
}
