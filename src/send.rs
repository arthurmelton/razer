use std::thread;
use std::thread::Thread;
use ws::util::Token;
use crate::event::event_type::Event;
use crate::listener::CONNECTIONS;

/// This code will send data to the website
/// ```
/// use razer::event::event_type::Event::JS;
/// use razer::send::send;
/// use razer::{Sender, Value};
/// fn load(&self, _event: Value, ctx: &Sender) {
///     if send(ctx, JS, "alert(\"Hello\")").is_err() {
///         println!("connection is closed");
///     }   
/// }
/// ```
/// this will fail if the connection was closed
pub fn send(client: &ws::Sender, event: Event, data: &str) -> Result<(), ()> {
    if crate::connections::closed(client) {
        return Err(());
    }
    if client
        .send(format!(
            "{{\"name\":\"{}\", \"data\":\"{}\"}}",
            event,
            data.replace("\"", "\\\"")
        ))
        .is_ok()
    {
        return Ok(());
    } else {
        return Err(());
    }
}

/// This code will send data to a specific client that could be different than the one who made the event
/// ```
/// use razer::event::event_type::Event::JS;
/// use razer::send::send_to;
/// use razer::{Sender, Value};
/// fn load(&self, _event: Value, ctx: &Sender) {
///     if send_to(ctx, 0, JS, "alert(\"Hello\")").is_err() {
///         println!("connection is closed");
///     }   
/// }
/// ```
/// this will send the alert to the first client that ever went onto the website <br />
/// to get the current id you can use `ctx.connection_id`
/// this will fail if the connection was closed
pub fn send_to(client: &ws::Sender, token: u32, event:Event, data: &str) -> Result<(),()> {
    let mut new_client= client.clone();
    new_client.change_token(token);
    if crate::connections::closed(&new_client) {
        return Err(());
    }
    send(&new_client, event, data)
}

/// This code will send a message to every client
/// ```
/// use razer::event::event_type::Event::JS;
/// use razer::send::broadcast;
/// use razer::{Sender, Value};
/// fn load(&self, _event: Value, ctx: &Sender) {
///     broadcast(ctx, JS, "alert(\"Hello\")");
/// }
/// ```
/// this will send the alert to every client that is currently on the site <br />
pub fn broadcast(client: &ws::Sender, event:Event, data: &str) {
    let mut threads = Vec::new();
    let connections = CONNECTIONS.lock().unwrap();
    for (i, _) in connections.clone().into_iter() {
        let sends = data.clone().to_string();
        let mut new_client= client.clone();
        threads.push(thread::spawn(move|| {
            new_client.change_token(i);
            if crate::connections::closed(&new_client) {
                return Err(());
            }
            send(&new_client, event, &*sends)
        }));
    }
    for h in threads {
        h.join().unwrap();
    }
}
