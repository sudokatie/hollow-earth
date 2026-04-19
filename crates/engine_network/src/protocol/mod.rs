//! Network protocol messages for client-server communication.

pub mod client_message;
pub mod hollow_earth_ext;
pub mod server_message;

pub use client_message::{ClientMessage, InputState};
pub use hollow_earth_ext::{
    entity_kind_name, HollowEarthClientMessage, HollowEarthEntityKind, HollowEarthServerMessage,
};
pub use server_message::{EntityKind, EntitySnapshot, ServerMessage, WorldSnapshot};
