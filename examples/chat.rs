use std::sync::Mutex;

use lazy_static::lazy_static;

use razer::event::event_type::Event::{EVAL, JS};
use razer::event::handler::EventHandler;
use razer::listener::Listener;
use razer::send::{broadcast, send};
use razer::Sender;
use razer::Value;

lazy_static! {
    static ref MESSAGES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[derive(Copy)]
pub struct Handler;

impl Clone for Handler {
    fn clone(&self) -> Self {
        *self
    }
}

impl EventHandler for Handler {
    fn load(&self, _event: Value, ctx: &Sender) {
        send(
            ctx,
            JS,
            &format!(
                "document.getElementById(\"text\").innerHTML += \"<p>{}</p>\"",
                MESSAGES.lock().unwrap().join("</p><p>")
            ),
        )
            .unwrap();
    }

    fn keydown(&self, event: Value, ctx: &Sender) {
        if event["key"].as_str().unwrap() == "Enter" && event["srcElement"]["id"] == "input" {
            send(ctx, EVAL, "document.getElementById(\"input\").value").unwrap();
            send(ctx, JS, "document.getElementById(\"input\").value = \"\"").unwrap();
        }
    }

    fn eval(&self, event: Value, ctx: &Sender) {
        MESSAGES
            .lock()
            .unwrap()
            .push(event["event"].as_str().unwrap().to_string());
        broadcast(
            ctx,
            JS,
            &format!(
                "document.getElementById(\"text\").innerHTML += \"<p>{}</p>\"",
                event["event"].as_str().unwrap().replace("\"", "\\\\\"")
            ),
        );
    }
}

fn main() {
    Listener::new().start(Handler);
}
