pub fn closed(ctx: &ws::Sender) -> bool {
    !crate::listener::CONNECTIONS
        .lock()
        .unwrap()
        .contains_key(&ctx.connection_id())
}

pub fn open(ctx: &ws::Sender) -> bool {
    crate::listener::CONNECTIONS
        .lock()
        .unwrap()
        .contains_key(&ctx.connection_id())
}
