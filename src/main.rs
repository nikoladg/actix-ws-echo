use actix::prelude::*;
use actix_web::{server, App, HttpRequest, HttpResponse};
use actix_web::ws;
use actix_web::error::{Error as ActixError};

fn ws_index(req: HttpRequest) -> Result<HttpResponse, ActixError> {
    ws::start(&req, WsEcho)
}

struct WsEcho;

impl Actor for WsEcho {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WsEcho {
    fn handle(&mut self, msg: ws::Message, _ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => {
                println!("{}", text);
            },
            _ => (),
        }
    }
}

fn main() {
    server::new(move || {
        App::new()
            .resource("/", move |r| {
                r.with(move |req| {
                    ws_index(req)
                })
            })
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
}

