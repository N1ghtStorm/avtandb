use crate::AppState;
use actix_send_websocket::{Message, WebSocket};
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddKVRequestDto {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetKVRequestDto {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KVResponceDto {
    pub error: String,
    pub value: String,
}

pub async fn add_kv_ws(data: web::Data<AppState>, ws: WebSocket) -> impl actix_web::Responder {
    init_ws_conn(data, ws, WsMethod::AddKvWs).await
}

pub async fn get_kv_ws(data: web::Data<AppState>, ws: WebSocket) -> impl actix_web::Responder {
    init_ws_conn(data, ws, WsMethod::GetKvWs).await
}

enum WsMethod {
    AddKvWs,
    GetKvWs,
    DeleteKvWs,
    UpdateKvWs,
}

async fn init_ws_conn(
    data: web::Data<AppState>,
    ws: WebSocket,
    method: WsMethod,
) -> impl actix_web::Responder {
    // stream is the async iterator of incoming client websocket messages.
    // res is the response we return to client.
    // tx is a sender to push new websocket message to client response.
    let (mut stream, res, tx) = ws.into_parts();
    // spawn the stream handling so we don't block the response to client.
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            let result = match msg {
                // we echo text message and ping message to client.
                Message::Text(text) => match method {
                    WsMethod::AddKvWs => {
                        let add_kv_request_dto_res: serde_json::Result<AddKVRequestDto> =
                            serde_json::from_str(&text);
                        let add_kv_request_dto = match add_kv_request_dto_res {
                            Err(e) => {
                                let _ = tx.text(format!("Error serializing {e}"));
                                continue;
                            }
                            Ok(a) => a,
                        };
                        let add_val_res = data
                            .kv_collection
                            .clone()
                            .add_value(add_kv_request_dto.key, add_kv_request_dto.value)
                            .await;
                        if let Err(_) = add_val_res {
                            let responce = KVResponceDto {
                                error: String::from("Add value error"),
                                value: String::from(""),
                            };
                            let answer = serde_json::to_string(&responce).expect("err serializing");
                            let _ = tx.text(answer);
                            continue;
                        }
                        let responce = KVResponceDto {
                            error: String::from(""),
                            value: String::from(""),
                        };
                        let answer = serde_json::to_string(&responce).expect("err serializing");
                        tx.text(answer)
                    }
                    WsMethod::GetKvWs => {
                        let get_kv_request_dto_res: serde_json::Result<GetKVRequestDto> =
                            serde_json::from_str(&text);
                        let get_kv_request_dto = match get_kv_request_dto_res {
                            Err(e) => {
                                let resp = KVResponceDto {
                                    error: format!("{e}"),
                                    value: String::from(""),
                                };
                                let _ =
                                    tx.text(serde_json::to_string(&resp).expect("err serializing"));
                                continue;
                            }
                            Ok(a) => a,
                        };
                        let get_val_res = match data
                            .kv_collection
                            .clone()
                            .get_value(get_kv_request_dto.key)
                            .await
                        {
                            Err(_) => {
                                let _ = tx.text("");
                                continue;
                            }
                            Ok(v) => v,
                        };
                        let responce = KVResponceDto {
                            error: String::from(""),
                            value: format!("{get_val_res}"),
                        };
                        let answer = serde_json::to_string(&responce).expect("err serializing");
                        tx.text(answer)
                    }
                    WsMethod::DeleteKvWs => {
                        todo!()
                    }
                    WsMethod::UpdateKvWs => todo!(),
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
