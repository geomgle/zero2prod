use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{query, Connection, PgPool};
use uuid::Uuid;

use crate::{FormData, Result};

#[tracing::instrument(
    name = "adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name)
    )
]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "saving new subscriber in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4);
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to execute query: {:#?}", e);
        e
    })?;
    Ok(())
}


#[tracing::instrument(
    name = "delete existing subscriber in the database",
    skip(form, pool)
)]
pub async fn delete_subscriber(pool: &PgPool, form: &FormData) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM subscriptions
        WHERE email = 'ursula_le_guin@gmail.com'
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to execute query: {:#?}", e);
        e
    })?;
    Ok(())
}

