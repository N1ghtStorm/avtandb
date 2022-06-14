use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_value(
    data: web::Data<AppState>,
    web::Path(key): web::Path<String>,
    value: String,
) -> impl Responder {
    if let Err(_) = data.kv_collection.clone().add_value(key, value).await {
        return HttpResponse::BadRequest().body("");
    }
    HttpResponse::Ok().body("")
}

pub async fn get_value(
    data: web::Data<AppState>,
    web::Path(key): web::Path<String>,
) -> impl Responder {
    let arc_string_value = match data.kv_collection.get_value(key).await {
        Ok(v) => v,
        Err(_) => return HttpResponse::NotFound().body(""),
    };
    HttpResponse::Ok().body(format!("{}", arc_string_value))
}

pub async fn update_value(
    data: web::Data<AppState>,
    web::Path(key): web::Path<String>,
    value: String,
) -> impl Responder {
    if let Err(_) = data.kv_collection.clone().update_value(key, value).await {
        return HttpResponse::BadRequest().body("");
    }
    HttpResponse::Ok().body("")
}

pub async fn delete_value(
    data: web::Data<AppState>,
    web::Path(key): web::Path<String>,
) -> impl Responder {
    if let Err(_) = data.kv_collection.clone().remove_value(key).await {
        return HttpResponse::BadRequest().body("");
    }
    HttpResponse::Ok().body("")
}

pub async fn get_all_keys(data: web::Data<AppState>) -> impl Responder {
    let keys = match data.kv_collection.clone().get_all_keys().await {
        Err(_) => return HttpResponse::BadRequest().body(""),
        Ok(v) => v,
    };
    let res = serde_json::to_vec(&keys).expect("deser err");
    HttpResponse::Ok().body(res)
}
