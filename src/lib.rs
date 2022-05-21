#![crate_type = "lib"]

extern crate websocket;

use std::thread;

pub use serde_json::Value;
use websocket::OwnedMessage;
pub use websocket::sender::Writer;
use websocket::sync::Server;

use crate::event::handler::EventHandler;

pub mod event;
pub mod send;

pub fn start<H: EventHandler + 'static + Copy>(handler: H) {
    let server = Server::bind("127.0.0.1:2794").unwrap();

    for request in server.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            if !request.protocols().contains(&"razer".to_string()) {
                request.reject().unwrap();
                return;
            }

            let client = request.use_protocol("razer").accept().unwrap();

            let ip = client.peer_addr().unwrap();

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();
                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }
                    OwnedMessage::Text(text) => {
                        let data: Value = serde_json::from_str(&text).unwrap();
                        match data["event_name"].as_str().unwrap() {
                            "abort" => handler.abort(data["event"].clone(), &mut sender),
                            "afterprint" => handler.afterprint(data["event"].clone(), &mut sender),
                            "animationend" => {
                                handler.animationend(data["event"].clone(), &mut sender)
                            }
                            "animationiteration" => {
                                handler.animationiteration(data["event"].clone(), &mut sender)
                            }
                            "animationstart" => {
                                handler.animationstart(data["event"].clone(), &mut sender)
                            }
                            "beforeprint" => {
                                handler.beforeprint(data["event"].clone(), &mut sender)
                            }
                            "beforeunload" => {
                                handler.beforeunload(data["event"].clone(), &mut sender)
                            }
                            "blur" => handler.blur(data["event"].clone(), &mut sender),
                            "canplay" => handler.canplay(data["event"].clone(), &mut sender),
                            "canplaythrough" => {
                                handler.canplaythrough(data["event"].clone(), &mut sender)
                            }
                            "change" => handler.change(data["event"].clone(), &mut sender),
                            "click" => handler.click(data["event"].clone(), &mut sender),
                            "contextmenu" => {
                                handler.contextmenu(data["event"].clone(), &mut sender)
                            }
                            "copy" => handler.copy(data["event"].clone(), &mut sender),
                            "cut" => handler.cut(data["event"].clone(), &mut sender),
                            "dblclick" => handler.dblclick(data["event"].clone(), &mut sender),
                            "drag" => handler.drag(data["event"].clone(), &mut sender),
                            "dragend" => handler.dragend(data["event"].clone(), &mut sender),
                            "dragenter" => handler.dragenter(data["event"].clone(), &mut sender),
                            "dragleave" => handler.dragleave(data["event"].clone(), &mut sender),
                            "dragover" => handler.dragover(data["event"].clone(), &mut sender),
                            "dragstart" => handler.dragstart(data["event"].clone(), &mut sender),
                            "drop" => handler.drop(data["event"].clone(), &mut sender),
                            "durationchange" => {
                                handler.durationchange(data["event"].clone(), &mut sender)
                            }
                            "ended" => handler.ended(data["event"].clone(), &mut sender),
                            "error" => handler.error(data["event"].clone(), &mut sender),
                            "focus" => handler.focus(data["event"].clone(), &mut sender),
                            "focusin" => handler.focusin(data["event"].clone(), &mut sender),
                            "focusout" => handler.focusout(data["event"].clone(), &mut sender),
                            "fullscreenchange" => {
                                handler.fullscreenchange(data["event"].clone(), &mut sender)
                            }
                            "fullscreenerror" => {
                                handler.fullscreenerror(data["event"].clone(), &mut sender)
                            }
                            "hashchange" => handler.hashchange(data["event"].clone(), &mut sender),
                            "input" => handler.input(data["event"].clone(), &mut sender),
                            "invalid" => handler.invalid(data["event"].clone(), &mut sender),
                            "keydown" => handler.keydown(data["event"].clone(), &mut sender),
                            "keypress" => handler.keypress(data["event"].clone(), &mut sender),
                            "keyup" => handler.keyup(data["event"].clone(), &mut sender),
                            "load" => handler.load(data["event"].clone(), &mut sender),
                            "loadeddata" => handler.loadeddata(data["event"].clone(), &mut sender),
                            "loadedmetadata" => {
                                handler.loadedmetadata(data["event"].clone(), &mut sender)
                            }
                            "loadstart" => handler.loadstart(data["event"].clone(), &mut sender),
                            "message" => handler.message(data["event"].clone(), &mut sender),
                            "mousedown" => handler.mousedown(data["event"].clone(), &mut sender),
                            "mouseenter" => handler.mouseenter(data["event"].clone(), &mut sender),
                            "mouseleave" => handler.mouseleave(data["event"].clone(), &mut sender),
                            "mousemove" => handler.mousemove(data["event"].clone(), &mut sender),
                            "mouseover" => handler.mouseover(data["event"].clone(), &mut sender),
                            "mouseout" => handler.mouseout(data["event"].clone(), &mut sender),
                            "mouseup" => handler.mouseup(data["event"].clone(), &mut sender),
                            "mousewheel" => handler.mousewheel(data["event"].clone(), &mut sender),
                            "offline" => handler.offline(data["event"].clone(), &mut sender),
                            "online" => handler.online(data["event"].clone(), &mut sender),
                            "open" => handler.open(data["event"].clone(), &mut sender),
                            "pagehide" => handler.pagehide(data["event"].clone(), &mut sender),
                            "pageshow" => handler.pageshow(data["event"].clone(), &mut sender),
                            "paste" => handler.paste(data["event"].clone(), &mut sender),
                            "pause" => handler.pause(data["event"].clone(), &mut sender),
                            "play" => handler.play(data["event"].clone(), &mut sender),
                            "playing" => handler.playing(data["event"].clone(), &mut sender),
                            "popstate" => handler.popstate(data["event"].clone(), &mut sender),
                            "progress" => handler.progress(data["event"].clone(), &mut sender),
                            "ratechange" => handler.ratechange(data["event"].clone(), &mut sender),
                            "resize" => handler.resize(data["event"].clone(), &mut sender),
                            "reset" => handler.reset(data["event"].clone(), &mut sender),
                            "scroll" => handler.scroll(data["event"].clone(), &mut sender),
                            "search" => handler.search(data["event"].clone(), &mut sender),
                            "seeked" => handler.seeked(data["event"].clone(), &mut sender),
                            "seeking" => handler.seeking(data["event"].clone(), &mut sender),
                            "select" => handler.select(data["event"].clone(), &mut sender),
                            "show" => handler.show(data["event"].clone(), &mut sender),
                            "stalled" => handler.stalled(data["event"].clone(), &mut sender),
                            "storage" => handler.storage(data["event"].clone(), &mut sender),
                            "submit" => handler.submit(data["event"].clone(), &mut sender),
                            "suspend" => handler.suspend(data["event"].clone(), &mut sender),
                            "timeupdate" => handler.timeupdate(data["event"].clone(), &mut sender),
                            "toggle" => handler.toggle(data["event"].clone(), &mut sender),
                            "touchcancel" => {
                                handler.touchcancel(data["event"].clone(), &mut sender)
                            }
                            "touchend" => handler.touchend(data["event"].clone(), &mut sender),
                            "touchmove" => handler.touchmove(data["event"].clone(), &mut sender),
                            "touchstart" => handler.touchstart(data["event"].clone(), &mut sender),
                            "transitionend" => {
                                handler.transitionend(data["event"].clone(), &mut sender)
                            }
                            "unload" => handler.unload(data["event"].clone(), &mut sender),
                            "volumechange" => {
                                handler.volumechange(data["event"].clone(), &mut sender)
                            }
                            "waiting" => handler.waiting(data["event"].clone(), &mut sender),
                            "wheel" => handler.wheel(data["event"].clone(), &mut sender),
                            _ => {}
                        };
                    }
                    _ => sender.send_message(&message).unwrap(),
                }
            }
        });
    }
}
