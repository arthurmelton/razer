
/// This will return a bool on if the connection is closed
/// ```
/// use razer::Sender;
/// use razer::connections::closed;
/// assert_eq!(closed(ctx), false);
/// // connection was closed here
/// assert_eq!(closed(ctx), true);
/// ```
pub fn closed(ctx: &ws::Sender) -> bool {
    !crate::listener::CONNECTIONS
        .lock()
        .unwrap()
        .contains_key(&ctx.token().0)
}

/// This will return a bool on if the connection is open
/// ```
/// use razer::Sender;
/// use razer::connections::open;
/// assert_eq!(open(ctx), true);
/// // connection was closed here
/// assert_eq!(open(ctx), false);
/// ```
pub fn open(ctx: &ws::Sender) -> bool {
    crate::listener::CONNECTIONS
        .lock()
        .unwrap()
        .contains_key(&ctx.token().0)
}
