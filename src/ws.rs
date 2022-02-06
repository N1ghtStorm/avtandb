use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;


struct AvtanKeeperConnector;

impl Actor for AvtanKeeperConnector {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AvtanKeeperConnector {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            // Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Text(text)) => {
                // ctx.text(text)
                let new_text = format!("hello {text}");
                ctx.text(new_text)
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(AvtanKeeperConnector {}, &req, stream);
    // println!("{:?}", resp);
    resp
}