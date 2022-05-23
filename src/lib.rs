#![crate_type = "lib"]

pub use serde_json::Value;
pub use ws::Sender;

use crate::event::handler::EventHandler;

pub mod connections;
pub mod event;
pub mod listener;
pub mod send;
