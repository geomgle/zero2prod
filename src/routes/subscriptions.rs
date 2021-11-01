use std::ops::Sub;

use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{query, Connection, PgPool};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::{
    domain::{NewSubscriber, SubscriberName},
    FormData,
    Result,
};

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
    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name,
    };
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "saving new subscriber in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4);
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
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
