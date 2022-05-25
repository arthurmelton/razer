use razer::event::event_type::Event::{EVAL, JS};
use razer::event::handler::EventHandler;
use razer::listener::Listener;
use razer::send::{broadcast, send};
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
            send(ctx, EVAL, "document.getElementById(\"input\").value").unwrap();
            send(ctx, JS, "document.getElementById(\"input\").value = \"\"").unwrap();
        }
    }

    fn eval(&self, event: Value, ctx: &Sender) {
        broadcast(
            ctx,
            JS,
            &format!(
                "document.getElementById(\"text\").innerHTML += \"<p>{}</p>\"",
                event["event"].as_str().unwrap()
            ),
        );
    }
}

fn main() {
    Listener::new().start(Handler);
}
