use std::net::TcpStream;

use websocket::{OwnedMessage, WebSocketResult};
use websocket::sender::Writer;

use crate::event::event_type::Event;

pub fn send(client: &mut Writer<TcpStream>, event: Event, data: String) -> WebSocketResult<()> {
    client.send_message(&OwnedMessage::Text(format!(
        "{{\"name\":\"{}\", \"data\":\"{}\"}}",
        event,
        data.replace("\"", "\\\"")
    )))
}
