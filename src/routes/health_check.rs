use actix_web::Responder;

pub async fn health_check() -> impl Responder {
    "webserver is running well"
}
