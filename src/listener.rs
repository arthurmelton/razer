use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

use lazy_static::lazy_static;
use openssl::pkey::{PKey, Private};
use openssl::ssl::{SslAcceptor, SslMethod, SslStream};
use openssl::x509::X509;
use serde_json::Value;
use ws::{CloseCode, Handshake};
use ws::util::TcpStream;

use crate::EventHandler;

#[derive(Clone, Debug)]
pub struct Lister {
    cert: Option<X509>,
    key: Option<PKey<Private>>,
    port: u16,
}

struct Server<H: EventHandler + 'static + Copy> {
    out: ws::Sender,
    handler: H,
    ssl: Option<Rc<SslAcceptor>>,
}

lazy_static! {
    pub static ref CONNECTIONS: Mutex<HashMap<u32, bool>> = Mutex::new(HashMap::new());
}

impl<H: EventHandler + 'static + Copy> ws::Handler for Server<H> {
    fn upgrade_ssl_server(&mut self, sock: TcpStream) -> ws::Result<SslStream<TcpStream>> {
        self.ssl.clone().unwrap().accept(sock).map_err(From::from)
    }

    fn on_open(&mut self, _shake: Handshake) -> ws::Result<()> {
        CONNECTIONS
            .lock()
            .unwrap()
            .insert(self.out.connection_id(), false);
        Ok(())
    }

    fn on_close(&mut self, _code: CloseCode, _reason: &str) {
        CONNECTIONS
            .lock()
            .unwrap()
            .remove(&self.out.connection_id());
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let handler = self.handler;
        let out = self.out.clone();
        thread::spawn(move || {
            let data: Value = serde_json::from_str(&msg.to_string()).unwrap();
            match data["event_name"].as_str().unwrap() {
                "abort" => handler.abort(data["event"].clone(), &out),
                "afterprint" => handler.afterprint(data["event"].clone(), &out),
                "animationend" => handler.animationend(data["event"].clone(), &out),
                "animationiteration" => handler.animationiteration(data["event"].clone(), &out),
                "animationstart" => handler.animationstart(data["event"].clone(), &out),
                "beforeprint" => handler.beforeprint(data["event"].clone(), &out),
                "beforeunload" => handler.beforeunload(data["event"].clone(), &out),
                "blur" => handler.blur(data["event"].clone(), &out),
                "canplay" => handler.canplay(data["event"].clone(), &out),
                "canplaythrough" => handler.canplaythrough(data["event"].clone(), &out),
                "change" => handler.change(data["event"].clone(), &out),
                "click" => handler.click(data["event"].clone(), &out),
                "contextmenu" => handler.contextmenu(data["event"].clone(), &out),
                "copy" => handler.copy(data["event"].clone(), &out),
                "cut" => handler.cut(data["event"].clone(), &out),
                "dblclick" => handler.dblclick(data["event"].clone(), &out),
                "drag" => handler.drag(data["event"].clone(), &out),
                "dragend" => handler.dragend(data["event"].clone(), &out),
                "dragenter" => handler.dragenter(data["event"].clone(), &out),
                "dragleave" => handler.dragleave(data["event"].clone(), &out),
                "dragover" => handler.dragover(data["event"].clone(), &out),
                "dragstart" => handler.dragstart(data["event"].clone(), &out),
                "drop" => handler.drop(data["event"].clone(), &out),
                "durationchange" => handler.durationchange(data["event"].clone(), &out),
                "ended" => handler.ended(data["event"].clone(), &out),
                "error" => handler.error(data["event"].clone(), &out),
                "focus" => handler.focus(data["event"].clone(), &out),
                "focusin" => handler.focusin(data["event"].clone(), &out),
                "focusout" => handler.focusout(data["event"].clone(), &out),
                "fullscreenchange" => handler.fullscreenchange(data["event"].clone(), &out),
                "fullscreenerror" => handler.fullscreenerror(data["event"].clone(), &out),
                "hashchange" => handler.hashchange(data["event"].clone(), &out),
                "input" => handler.input(data["event"].clone(), &out),
                "invalid" => handler.invalid(data["event"].clone(), &out),
                "keydown" => handler.keydown(data["event"].clone(), &out),
                "keypress" => handler.keypress(data["event"].clone(), &out),
                "keyup" => handler.keyup(data["event"].clone(), &out),
                "load" => handler.load(data["event"].clone(), &out),
                "loadeddata" => handler.loadeddata(data["event"].clone(), &out),
                "loadedmetadata" => handler.loadedmetadata(data["event"].clone(), &out),
                "loadstart" => handler.loadstart(data["event"].clone(), &out),
                "message" => handler.message(data["event"].clone(), &out),
                "mousedown" => handler.mousedown(data["event"].clone(), &out),
                "mouseenter" => handler.mouseenter(data["event"].clone(), &out),
                "mouseleave" => handler.mouseleave(data["event"].clone(), &out),
                "mousemove" => handler.mousemove(data["event"].clone(), &out),
                "mouseover" => handler.mouseover(data["event"].clone(), &out),
                "mouseout" => handler.mouseout(data["event"].clone(), &out),
                "mouseup" => handler.mouseup(data["event"].clone(), &out),
                "mousewheel" => handler.mousewheel(data["event"].clone(), &out),
                "offline" => handler.offline(data["event"].clone(), &out),
                "online" => handler.online(data["event"].clone(), &out),
                "open" => handler.open(data["event"].clone(), &out),
                "pagehide" => handler.pagehide(data["event"].clone(), &out),
                "pageshow" => handler.pageshow(data["event"].clone(), &out),
                "paste" => handler.paste(data["event"].clone(), &out),
                "pause" => handler.pause(data["event"].clone(), &out),
                "play" => handler.play(data["event"].clone(), &out),
                "playing" => handler.playing(data["event"].clone(), &out),
                "popstate" => handler.popstate(data["event"].clone(), &out),
                "progress" => handler.progress(data["event"].clone(), &out),
                "ratechange" => handler.ratechange(data["event"].clone(), &out),
                "resize" => handler.resize(data["event"].clone(), &out),
                "reset" => handler.reset(data["event"].clone(), &out),
                "scroll" => handler.scroll(data["event"].clone(), &out),
                "search" => handler.search(data["event"].clone(), &out),
                "seeked" => handler.seeked(data["event"].clone(), &out),
                "seeking" => handler.seeking(data["event"].clone(), &out),
                "select" => handler.select(data["event"].clone(), &out),
                "show" => handler.show(data["event"].clone(), &out),
                "stalled" => handler.stalled(data["event"].clone(), &out),
                "storage" => handler.storage(data["event"].clone(), &out),
                "submit" => handler.submit(data["event"].clone(), &out),
                "suspend" => handler.suspend(data["event"].clone(), &out),
                "timeupdate" => handler.timeupdate(data["event"].clone(), &out),
                "toggle" => handler.toggle(data["event"].clone(), &out),
                "touchcancel" => handler.touchcancel(data["event"].clone(), &out),
                "touchend" => handler.touchend(data["event"].clone(), &out),
                "touchmove" => handler.touchmove(data["event"].clone(), &out),
                "touchstart" => handler.touchstart(data["event"].clone(), &out),
                "transitionend" => handler.transitionend(data["event"].clone(), &out),
                "unload" => handler.unload(data["event"].clone(), &out),
                "volumechange" => handler.volumechange(data["event"].clone(), &out),
                "waiting" => handler.waiting(data["event"].clone(), &out),
                "wheel" => handler.wheel(data["event"].clone(), &out),
                _ => {}
            };
        });
        Ok(())
    }
}

impl Lister {
    #[must_use]
    pub fn new() -> Lister {
        return Lister {
            cert: None,
            key: None,
            port: 2794,
        };
    }

    pub fn with_key(&self, key: &str) -> Lister {
        return Lister {
            cert: self.clone().cert,
            key: {
                let data = read_file(key).unwrap();
                Some(PKey::private_key_from_pem(data.as_ref()).unwrap())
            },
            port: self.clone().port,
        };
    }

    pub fn with_cert(&self, cert: &str) -> Lister {
        return Lister {
            cert: {
                let data = read_file(cert).unwrap();
                Some(X509::from_pem(data.as_ref()).unwrap())
            },
            key: self.clone().key,
            port: self.clone().port,
        };
    }

    pub fn start<H: EventHandler + 'static + Copy>(&self, handler: H) {
        if self.key.is_some() && self.cert.is_some() {
            let acceptor = Rc::new({
                let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
                builder.set_private_key(&self.clone().key.unwrap()).unwrap();
                builder
                    .set_certificate(&self.clone().cert.unwrap())
                    .unwrap();

                builder.build()
            });

            ws::Builder::new()
                .with_settings(ws::Settings {
                    encrypt_server: true,
                    ..ws::Settings::default()
                })
                .build(|out: ws::Sender| Server {
                    out,
                    handler,
                    ssl: Some(acceptor.clone()),
                })
                .unwrap()
                .listen(format!("0.0.0.0:{}", self.port))
                .unwrap();
        } else {
            ws::Builder::new()
                .with_settings(ws::Settings {
                    ..ws::Settings::default()
                })
                .build(|out: ws::Sender| Server {
                    out,
                    handler,
                    ssl: None,
                })
                .unwrap()
                .listen(format!("0.0.0.0:{}", self.port))
                .unwrap();
        };
    }
}

fn read_file(name: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(name)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
