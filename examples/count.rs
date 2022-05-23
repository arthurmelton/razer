use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;

use razer::event::event_type::Event::JS;
use razer::event::handler::EventHandler;
use razer::listener::Lister;
use razer::send::send;
use razer::Sender;
use razer::Value;

lazy_static! {
    static ref DATA: Mutex<u32> = Mutex::new(0);
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
        while send(
            &ctx,
            JS,
            format!(
                "document.getElementById(\"counter\").innerHTML = {}",
                DATA.lock().unwrap()
            )
                .as_str(),
        )
            .is_ok()
        {
            sleep(Duration::from_secs(1));
        }
    }

    fn click(&self, _event: Value, ctx: &Sender) {
        let mut counter = DATA.lock().unwrap();
        *counter += 1;
        send(
            &ctx,
            JS,
            format!(
                "document.getElementById(\"counter\").innerHTML = {}",
                counter
            )
                .as_str(),
        )
            .unwrap();
    }
}

fn main() {
    Lister::new().start(Handler);
}
