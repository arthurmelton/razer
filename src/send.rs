use std::thread;

use razer_ws::util::Token;

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
pub fn send(client: &razer_ws::Sender, event: Event, data: &str) -> Result<(), ()> {
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
/// use razer::Token;
/// fn load(&self, _event: Value, ctx: &Sender) {
///     if send_to(ctx, Token::from(0), JS, "alert(\"Hello\")").is_err() {
///         println!("connection is closed");
///     }   
/// }
/// ```
/// this will send the alert to the first client that ever went onto the website <br />
/// to get the current id you can use `ctx.token()`
/// this will fail if the connection was closed
pub fn send_to(
    client: &razer_ws::Sender,
    token: Token,
    event: Event,
    data: &str,
) -> Result<(), ()> {
    let mut new_client = client.clone();
    let connections = CONNECTIONS.lock().unwrap();
    let connection = connections.clone();
    drop(connections);
    if connection.contains_key(&token.0) {
        new_client.change_token(token, *connection.get(&token.0).unwrap());
        if crate::connections::closed(&new_client) {
            return Err(());
        }
        send(&new_client, event, data)
    } else {
        Err(())
    }
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
pub fn broadcast(client: &razer_ws::Sender, event: Event, data: &str) {
    let connections = CONNECTIONS.lock().unwrap();
    let connection = connections.clone();
    let mut threads = Vec::new();
    drop(connections);
    for (i, _) in connection.into_iter() {
        let message = data.to_string();
        let client = client.clone();
        threads.push(thread::spawn(move || {
            let _ = send_to(&client, Token::from(i.clone()), event.clone(), &message);
        }));
    }
    for i in threads {
        i.join().unwrap()
    }
}
