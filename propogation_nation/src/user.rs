// theres a lot of user stuff in auth for onboarding, but the idea is they can optionally provide
// more details for a social media style account, and people can view their profile.
use anyhow::{anyhow, Result};
use leptos::prelude::ServerFnError;
use leptos::prelude::*;
use leptos::server;
use leptos::task::spawn_local;
use leptos::Params;
use leptos_router::hooks::{use_params, use_query};
use leptos_router::params::Params;
#[cfg(feature = "ssr")]
use serde::Deserialize;
use serde::Serialize;

#[server]
pub async fn add_user_details(
    session_id: String,
    located_at: String,
    photo_url: String,
) -> Result<(), ServerFnError> {
    use crate::types::AppState;
    use crate::types::UserDetails;
    use crate::types::UserSessionSS;
    use crate::utils::get_user_session;
    use axum::Extension;
    use leptos::prelude::*;
    use leptos_axum::extract;
    use lettre::message::{header::ContentType, Mailbox};
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    use sqlx::postgres::PgPool;
    use std::sync::Arc;
    use uuid::Uuid;

    let user_session_ss = get_user_session(session_id).await?;

    let Extension(app_state): Extension<Arc<AppState>> = extract().await?;
    let pool = &app_state.pool;
    sqlx::query!(
        "INSERT INTO user_details (id, photo_url, location) VALUES ($1, $2, $3)",
        user_session_ss.id,
        photo_url.as_str(),
        located_at.as_str()
    )
    .execute(pool)
    .await?;
    leptos_axum::redirect("/homepage");
    Ok(())
}
