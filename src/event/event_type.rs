use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    JS,
    HTML,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::JS => write!(f, "js"),
            Event::HTML => write!(f, "html"),
        }
    }
}
