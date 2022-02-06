use core::time;
use std::{sync::Arc, pin::Pin};
use serde::{Serialize, Deserialize};
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::future::Future;



// use actix::{fut, ActorContext};
// // use crate::messages::{Disconnect, Connect, WsMessage, ClientActorMessage}; //We'll be writing this later
// // use crate::lobby::Lobby; // as well as this
// use actix::{Addr, Running, WrapFuture, ActorFuture, ContextFutureSpawner};
// use actix::{AsyncContext, Handler};
// // use actix_web_actors::ws;
// use actix_web_actors::ws::Message::Text;
// use std::time::{Duration, Instant};
// use uuid::Uuid;





use crate::{AppState, kv_model::InMemoryKVStore};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddKVRequestDto {
    pub key: String, 
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetKVRequestDto {
    pub key: String, 
}

// pub struct DeleteKVRequestDto {
//     pub key: String, 
// }

// pub struct UpdateKVRequestDto {
//     pub key: String, 
//     pub value: String,
// }

struct AddKVRequestActor {
    pub kv_store: InMemoryKVStore
}

impl Actor for AddKVRequestActor {
    type Context = ws::WebsocketContext<Self>;

    // fn started(&mut self, ctx: &mut Self::Context) {
    //     // self.hb(ctx);

    //     let addr = ctx.address();
    //     self.lobby_addr
    //         .send(Connect {
    //             addr: addr.recipient(),
    //             lobby_id: self.room,
    //             self_id: self.id,
    //         })
    //         .into_actor(self)
    //         .then(|res, _, ctx| {
    //             match res {
    //                 Ok(_res) => (),
    //                 _ => ctx.stop(),
    //             }
    //             fut::ready(())
    //         })
    //         .wait(ctx);
    // }
}

use std::sync::mpsc::{Sender, Receiver};

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AddKVRequestActor {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let add_kv_request_dto_res: serde_json::Result<AddKVRequestDto> =  serde_json::from_str(&text);
                let add_kv_request_dto = match add_kv_request_dto_res{
                    Err(e) => return ctx.text(format!("Error serializing {e}")),
                    Ok(a) => a
                };

                // let future = async move {
                //     self.kv_store.clone().add_value(add_kv_request_dto.key, add_kv_request_dto.value).await
                // };
                // let res = tokio::runtime::Builder::new()
                //         .basic_scheduler()
                //         .enable_all()
                //         .build()
                //         .unwrap()
                //         .block_on(future);

                // let v = sexecutor::block_on( self.kv_store.clone().add_value(add_kv_request_dto.key, add_kv_request_dto.value))

                // let res = actix::fut::wrap_future::<_, Self>(future);
                // ctx.spawn(res);
                // let (mut tx, mut rx) =  tokio::sync::mpsc::channel(100);

                let ptr = ctx as *mut Self::Context;

                let s = Arc::new(tokio::sync::RwLock::new(unsafe {std::ptr::read(ptr)}));
                // let ctx_copy_u = 0 as *mut Self::Context;
                // unsafe { std::ptr::write(ptr, s) };
                // let aaa = ctx_copy_u as *mut u8;
                // // let ctx_copy = unsafe {&mut std::ptr::read(ctx_copy_u)};
                // let arc = Arc::new(aaa);

                let (tx, rx) = std::sync::mpsc::channel();
                let mut kv_clone = self.kv_store.clone();
                let tx1 = tx.clone();

                // let a = std::thread::spawn(|| {

                // });

                let join_handle = tokio::spawn(async move {
                    let v = kv_clone.clone();
                    let res = kv_clone.add_value(add_kv_request_dto.key, add_kv_request_dto.value).await;
                    let send_res = tx1.send(res);
                    if let Err(e) = send_res {
                        println!("tx1 channel write error: {e}")
                    }
                    // let a = s.clone();

                });

                // let a = join_handle.;
                std::thread::sleep(time::Duration::from_millis(10000));
                if let Err(e) = tx.send(Ok(())) {
                    println!("tx channel write error: {e}")
                }
                // let u = true;

                // loop {
                //     if rx.iter().next().is_some() {
                //         break;
                //     }
                //     std::thread::sleep(time::Duration::from_millis(500));
                // }
                // let u = rx.iter().next().unwrap();
                // let u = rx.iter().next().unwrap();

                
                ctx.text("Ok");

                // let pin = Pin::new(Box::new(join_handle)); 
                // let a = pin.poll();
                // let a = join_handle.poll();


                // a.join();
                // if let Err(_) = res {
                //     return ctx.text(format!("Tokio futures error"))
                // }
                // let mut i = 0;
                // std::thread::sleep(time::Duration::from_millis(2000));
                // while let Err(e) = rx.try_recv() {
                //     // i += 1;
                //     // println!("{i} {e}")
                //     std::thread::sleep(time::Duration::from_millis(2000))
                // }

                ctx.text("Ok")
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn add_kv_ws(data: web::Data<AppState>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(AddKVRequestActor{ kv_store: data.kv_collection.clone() }, &req, stream);
    resp
}


// use actix_web::{get, App, Error, HttpRequest, HttpServer, Responder};
use actix_send_websocket::{Message, WebSocket};

pub async fn add_kv_ws2(data: web::Data<AppState>, ws: WebSocket) -> impl actix_web::Responder {
    // stream is the async iterator of incoming client websocket messages.
    // res is the response we return to client.
    // tx is a sender to push new websocket message to client response.
    let (mut stream, res, mut tx) = ws.into_parts();

    // spawn the stream handling so we don't block the response to client.
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            let result = match msg {
                // we echo text message and ping message to client.
                Message::Text(text) => {
                    let add_kv_request_dto_res: serde_json::Result<AddKVRequestDto> =  serde_json::from_str(&text);
                    let add_kv_request_dto = match add_kv_request_dto_res{
                        Err(e) => {
                            tx.text(format!("Error serializing {e}"));
                            return;
                        },
                        Ok(a) => a
                    };
                    let res = data.kv_collection.clone().add_value(add_kv_request_dto.key, add_kv_request_dto.value).await;
                    if let Err(e) = res {
                        let _ = tx.text("bad!!!");
                    }

                    tx.text("ok")
                },
                Message::Ping(bytes) => tx.pong(&bytes),
                Message::Close(reason) => {
                    println!("{}", reason.clone().unwrap().description.unwrap());
                    let _ = tx.close(reason);
                    // force end the stream when we have a close message.
                    break;
                }
                // other types of message would be ignored
                _ => Ok(()),
            };
            if result.is_err() {
                // end the stream when the response is gone.
                break;
            }
        }   
    });

    res
}