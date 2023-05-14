use actix_web::{Responder, HttpResponse};
use crate::models::user::User;

pub async fn get_all_users () -> impl Responder {
    let data: Vec<User> = User::get_all_users();
    let data_output_format = serde_json::json!({
        "status": "success",
        "data": data
    });
    HttpResponse::Ok().json(data_output_format)
}
