use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use openssl::pkey::{PKey, Private};
use openssl::ssl::{SslAcceptor, SslMethod, SslStream};
use openssl::x509::X509;
use serde_json::Value;
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

impl<H: EventHandler + 'static + Copy> ws::Handler for Server<H> {
    fn upgrade_ssl_server(&mut self, sock: TcpStream) -> ws::Result<SslStream<TcpStream>> {
        self.ssl.clone().unwrap().accept(sock).map_err(From::from)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let data: Value = serde_json::from_str(&msg.to_string()).unwrap();
        match data["event_name"].as_str().unwrap() {
            "abort" => self.handler.abort(data["event"].clone(), &self.out),
            "afterprint" => self.handler.afterprint(data["event"].clone(), &self.out),
            "animationend" => self.handler.animationend(data["event"].clone(), &self.out),
            "animationiteration" => self
                .handler
                .animationiteration(data["event"].clone(), &self.out),
            "animationstart" => self
                .handler
                .animationstart(data["event"].clone(), &self.out),
            "beforeprint" => self.handler.beforeprint(data["event"].clone(), &self.out),
            "beforeunload" => self.handler.beforeunload(data["event"].clone(), &self.out),
            "blur" => self.handler.blur(data["event"].clone(), &self.out),
            "canplay" => self.handler.canplay(data["event"].clone(), &self.out),
            "canplaythrough" => self
                .handler
                .canplaythrough(data["event"].clone(), &self.out),
            "change" => self.handler.change(data["event"].clone(), &self.out),
            "click" => self.handler.click(data["event"].clone(), &self.out),
            "contextmenu" => self.handler.contextmenu(data["event"].clone(), &self.out),
            "copy" => self.handler.copy(data["event"].clone(), &self.out),
            "cut" => self.handler.cut(data["event"].clone(), &self.out),
            "dblclick" => self.handler.dblclick(data["event"].clone(), &self.out),
            "drag" => self.handler.drag(data["event"].clone(), &self.out),
            "dragend" => self.handler.dragend(data["event"].clone(), &self.out),
            "dragenter" => self.handler.dragenter(data["event"].clone(), &self.out),
            "dragleave" => self.handler.dragleave(data["event"].clone(), &self.out),
            "dragover" => self.handler.dragover(data["event"].clone(), &self.out),
            "dragstart" => self.handler.dragstart(data["event"].clone(), &self.out),
            "drop" => self.handler.drop(data["event"].clone(), &self.out),
            "durationchange" => self
                .handler
                .durationchange(data["event"].clone(), &self.out),
            "ended" => self.handler.ended(data["event"].clone(), &self.out),
            "error" => self.handler.error(data["event"].clone(), &self.out),
            "focus" => self.handler.focus(data["event"].clone(), &self.out),
            "focusin" => self.handler.focusin(data["event"].clone(), &self.out),
            "focusout" => self.handler.focusout(data["event"].clone(), &self.out),
            "fullscreenchange" => self
                .handler
                .fullscreenchange(data["event"].clone(), &self.out),
            "fullscreenerror" => self
                .handler
                .fullscreenerror(data["event"].clone(), &self.out),
            "hashchange" => self.handler.hashchange(data["event"].clone(), &self.out),
            "input" => self.handler.input(data["event"].clone(), &self.out),
            "invalid" => self.handler.invalid(data["event"].clone(), &self.out),
            "keydown" => self.handler.keydown(data["event"].clone(), &self.out),
            "keypress" => self.handler.keypress(data["event"].clone(), &self.out),
            "keyup" => self.handler.keyup(data["event"].clone(), &self.out),
            "load" => self.handler.load(data["event"].clone(), &self.out),
            "loadeddata" => self.handler.loadeddata(data["event"].clone(), &self.out),
            "loadedmetadata" => self
                .handler
                .loadedmetadata(data["event"].clone(), &self.out),
            "loadstart" => self.handler.loadstart(data["event"].clone(), &self.out),
            "message" => self.handler.message(data["event"].clone(), &self.out),
            "mousedown" => self.handler.mousedown(data["event"].clone(), &self.out),
            "mouseenter" => self.handler.mouseenter(data["event"].clone(), &self.out),
            "mouseleave" => self.handler.mouseleave(data["event"].clone(), &self.out),
            "mousemove" => self.handler.mousemove(data["event"].clone(), &self.out),
            "mouseover" => self.handler.mouseover(data["event"].clone(), &self.out),
            "mouseout" => self.handler.mouseout(data["event"].clone(), &self.out),
            "mouseup" => self.handler.mouseup(data["event"].clone(), &self.out),
            "mousewheel" => self.handler.mousewheel(data["event"].clone(), &self.out),
            "offline" => self.handler.offline(data["event"].clone(), &self.out),
            "online" => self.handler.online(data["event"].clone(), &self.out),
            "open" => self.handler.open(data["event"].clone(), &self.out),
            "pagehide" => self.handler.pagehide(data["event"].clone(), &self.out),
            "pageshow" => self.handler.pageshow(data["event"].clone(), &self.out),
            "paste" => self.handler.paste(data["event"].clone(), &self.out),
            "pause" => self.handler.pause(data["event"].clone(), &self.out),
            "play" => self.handler.play(data["event"].clone(), &self.out),
            "playing" => self.handler.playing(data["event"].clone(), &self.out),
            "popstate" => self.handler.popstate(data["event"].clone(), &self.out),
            "progress" => self.handler.progress(data["event"].clone(), &self.out),
            "ratechange" => self.handler.ratechange(data["event"].clone(), &self.out),
            "resize" => self.handler.resize(data["event"].clone(), &self.out),
            "reset" => self.handler.reset(data["event"].clone(), &self.out),
            "scroll" => self.handler.scroll(data["event"].clone(), &self.out),
            "search" => self.handler.search(data["event"].clone(), &self.out),
            "seeked" => self.handler.seeked(data["event"].clone(), &self.out),
            "seeking" => self.handler.seeking(data["event"].clone(), &self.out),
            "select" => self.handler.select(data["event"].clone(), &self.out),
            "show" => self.handler.show(data["event"].clone(), &self.out),
            "stalled" => self.handler.stalled(data["event"].clone(), &self.out),
            "storage" => self.handler.storage(data["event"].clone(), &self.out),
            "submit" => self.handler.submit(data["event"].clone(), &self.out),
            "suspend" => self.handler.suspend(data["event"].clone(), &self.out),
            "timeupdate" => self.handler.timeupdate(data["event"].clone(), &self.out),
            "toggle" => self.handler.toggle(data["event"].clone(), &self.out),
            "touchcancel" => self.handler.touchcancel(data["event"].clone(), &self.out),
            "touchend" => self.handler.touchend(data["event"].clone(), &self.out),
            "touchmove" => self.handler.touchmove(data["event"].clone(), &self.out),
            "touchstart" => self.handler.touchstart(data["event"].clone(), &self.out),
            "transitionend" => self.handler.transitionend(data["event"].clone(), &self.out),
            "unload" => self.handler.unload(data["event"].clone(), &self.out),
            "volumechange" => self.handler.volumechange(data["event"].clone(), &self.out),
            "waiting" => self.handler.waiting(data["event"].clone(), &self.out),
            "wheel" => self.handler.wheel(data["event"].clone(), &self.out),
            _ => {}
        };
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
