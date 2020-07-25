use actix_web::{HttpResponse, Result as HttpResult};

pub async fn parse_csv() -> HttpResult<HttpResponse> {
    Ok(HttpResponse::Ok().into())
}
