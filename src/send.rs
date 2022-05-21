use crate::event::event_type::Event;
use std::net::TcpStream;
use websocket::sender::Writer;
use websocket::{OwnedMessage, WebSocketResult};

pub fn send(client: &mut Writer<TcpStream>, event: Event, data: String) -> WebSocketResult<()> {
    client.send_message(&OwnedMessage::Text(format!(
        "{{\"name\":\"{}\", \"data\":\"{}\"}}",
        event,
        data.replace("\"", "\\\"")
    )))
}
