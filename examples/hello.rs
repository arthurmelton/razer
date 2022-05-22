use razer::event::event_type::Event::JS;
use razer::event::handler::EventHandler;
use razer::listener::Lister;
use razer::send::send;
use razer::Value;
use razer::Sender;

#[derive(Copy)]
pub struct Handler;

impl Clone for Handler {
    fn clone(&self) -> Self { *self }
}

impl EventHandler for Handler {
    fn load(&self, _event: Value, ctx: &Sender) {
        if let Err(why) = send(ctx, JS, "alert(\"Hello\")") {
            println!("Error: {}", why);
        }
    }
}

fn main() {
    Lister::new().start(Handler);
}
