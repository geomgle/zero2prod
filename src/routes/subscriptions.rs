use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{query, Connection, PgPool};
use tracing_futures::Instrument;
use uuid::Uuid;

use crate::FormData;

#[tracing::instrument(
    name = "adding a new subscriber",
    skip(form, pool),
    fields(request_id = %Uuid::new_v4(),
    subscriber_email = %form.email,
    subscriber_name = %form.name))
]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = uuid::Uuid::new_v4();
    let request_span = tracing::info_span!(
        "adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    let _req_span_guard = request_span.enter();

    let query_span =
        tracing::info_span!("saving new subscriber details in the database");
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
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - new subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - failed to execute query for inserting into subscriptions: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
