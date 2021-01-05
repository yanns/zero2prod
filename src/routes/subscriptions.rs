use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{NewSubscriber, SubscriberName};

#[derive(serde::Deserialize)]
pub struct SubscribeRequest {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(payload, pool),
    fields(
        request_id=%Uuid::new_v4(),
        email = %payload.email,
        name = %payload.name
    )
)]
pub async fn subscribe(
    payload: web::Form<SubscribeRequest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    if payload.email.trim().is_empty() {
        return Err(HttpResponse::BadRequest().finish());
    }
    let name = SubscriberName::parse(payload.0.name)
        .map_err(|_| HttpResponse::BadRequest().finish())?;
    let new_subscriber = NewSubscriber {
        email: payload.0.email,
        name,
    };
    insert_subscriber(&pool, &new_subscriber)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
