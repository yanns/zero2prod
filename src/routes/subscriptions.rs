use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use unicode_segmentation::UnicodeSegmentation;

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
    if !is_valid_name(&payload.name) {
        return Err(HttpResponse::BadRequest().finish());
    }
    if payload.email.trim().is_empty() {
        return Err(HttpResponse::BadRequest().finish());
    }
    insert_subscriber(&pool, &payload)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().finish())
}

pub fn is_valid_name(s: &str) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();
    let is_too_long = s.graphemes(true).count() > 256;
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(payload, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    payload: &SubscribeRequest,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        payload.email,
        payload.name,
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
