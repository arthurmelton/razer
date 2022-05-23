use crate::event::event_type::Event;

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
