use crate::event::event_type::Event;

pub fn send(client: &ws::Sender, event: Event, data: &str) -> ws::Result<()> {
    client.send(format!(
        "{{\"name\":\"{}\", \"data\":\"{}\"}}",
        event,
        data.replace("\"", "\\\"")
    ))
}
