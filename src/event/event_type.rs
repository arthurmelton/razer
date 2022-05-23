use std::fmt;

/// This will tell the sender what you want to do <br />
/// JS -> will run js on the website <br />
/// HTML -> will set the body tag
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
