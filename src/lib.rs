#![crate_type = "lib"]

//! Razer is serverside way to control your website with rust
//! # Example
//! ```
//!use razer::event::event_type::Event::JS;
//! use razer::event::handler::EventHandler;
//! use razer::listener::Listener;
//! use razer::send::send;
//! use razer::Sender;
//! use razer::Value;
//!
//! #[derive(Copy)]
//! pub struct Handler;
//!
//! impl Clone for Handler {
//!     fn clone(&self) -> Self {
//!         *self
//!     }
//! }
//!
//! impl EventHandler for Handler {
//!     fn load(&self, _event: Value, ctx: &Sender) {
//!         send(ctx, JS, "alert(\"Hello\")").unwrap();
//!     }
//! }
//!
//! fn main() {
//!     Listener::new().start(Handler);
//! }
//! ```
//! this will alert "Hello" when ever someone goes onto your site <br />
//! To add the script all you will have to do is add this to your html
//!
//! ```html
//! <script src="https://cdn.jsdelivr.net/gh/AMTitan/razer/js/razer.min.js"></script>
//! ```

pub use serde_json::Value;
pub use ws::Sender;
pub use ws::util::Token;

use crate::event::handler::EventHandler;

pub mod connections;
pub mod event;
pub mod listener;
pub mod send;
