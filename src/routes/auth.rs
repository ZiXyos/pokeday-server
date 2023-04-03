use actix_web::{post, Error, HttpResponse, web::Json};

use crate::accounts::types::AccountReq;

pub const APPLICATION_JSON: &str = "application/json";

#[post("/auth/create-account")]
pub async fn create_account(req: Json<AccountReq>) -> HttpResponse {
    
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(req.to_account())
}
