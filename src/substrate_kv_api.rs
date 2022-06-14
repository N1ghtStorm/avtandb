// use crate::{AppState};
// use actix_web::{web, HttpResponse, Responder};

// use sp_core::crypto::Pair;
// use sp_keyring::AccountKeyring;
// use sp_runtime::MultiAddress;

// use substrate_api_client::rpc::WsRpcClient;
// use substrate_api_client::{
//     compose_extrinsic, Api, GenericAddress, UncheckedExtrinsicV4, XtStatus,
// };


// pub async fn create_value(data: web::Data<AppState>, web::Path(key): web::Path<String>, value: String) -> impl Responder {
//     // if let Err(_) = data.kv_collection.clone().add_value(key, value).await {
//     //     return HttpResponse::BadRequest().body("")
//     // }
//     // instantiate an Api that connects to the given address

//     let url = "127.0.0.1:9944";
//     let from = AccountKeyring::Alice.pair();
//     let client = WsRpcClient::new(&url);
//     let api = Api::new(client)
//         .map(|api| api.set_signer(from.clone()))
//         .unwrap();


//     #[allow(clippy::redundant_clone)]
//     let xt: UncheckedExtrinsicV4<_> = compose_extrinsic!(
//         api.clone(),
//         "avtanDbKeyValue",
//         "addValue",
//         key,
//         value
//     );

//     HttpResponse::Ok().body("")
// }

// pub fn create_test_value() {
//     // if let Err(_) = data.kv_collection.clone().add_value(key, value).await {
//     //     return HttpResponse::BadRequest().body("")
//     // }
//     // instantiate an Api that connects to the given address

//     let key = "lol".to_string();
//     let value = "lol lol lol".to_string();

//     let url = "127.0.0.1:9944";
//     let from = AccountKeyring::Alice.pair();
//     let client = WsRpcClient::new(&url);
//     let api = Api::new(client)
//         .map(|api| api.set_signer(from.clone()))
//         .unwrap();


//     #[allow(clippy::redundant_clone)]
//     let xt: UncheckedExtrinsicV4<_> = compose_extrinsic!(
//         api.clone(),
//         "avtanDbKeyValue",
//         "addValue",
//         key,
//         value
//     );

//     // HttpResponse::Ok().body("")
// }