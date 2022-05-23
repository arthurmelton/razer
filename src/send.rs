use crate::event::event_type::Event;

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
