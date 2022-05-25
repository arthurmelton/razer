use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;

use razer::event::event_type::Event::{EVAL, JS};
use razer::event::handler::EventHandler;
use razer::listener::{CONNECTIONS, Listener};
use razer::send::{broadcast, send, send_to};
use razer::Sender;
use razer::Value;

#[derive(Copy)]
pub struct Handler;

impl Clone for Handler {
    fn clone(&self) -> Self {
        *self
    }
}

impl EventHandler for Handler {
    fn keydown(&self, event: Value, ctx: &Sender) {
        if event["key"].as_str().unwrap() == "Enter" {
            send(ctx, EVAL, "document.getElementById(\"input\").value");
            send(ctx, JS, "document.getElementById(\"input\").value = \"\"");
        }
    }

    fn eval(&self, event: Value, ctx: &Sender) {
        broadcast(ctx, JS, &format!("document.getElementById(\"text\").innerHTML += \"<p>{}</p>\"", event.as_str().unwrap()))
    }
}

fn main() {
    Listener::new().start(Handler);
}
