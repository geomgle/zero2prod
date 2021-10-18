use actix_web::{web, HttpResponse};
use chrono::Utc;
use log;
use sqlx::{query, Connection, PgPool};
use uuid::Uuid;

use crate::FormData;

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    log::info!("saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4);
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("new subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "failed to execute query for inserting into subscriptions"
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
