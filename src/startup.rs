use std::{net::TcpListener, sync::Arc};

use actix_web::{
    dev::Server,
    middleware::Logger,
    web,
    App,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Responder,
};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::{
    routes::{health_check, subscribe},
    Result,
};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
