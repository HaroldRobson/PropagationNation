use anyhow::{anyhow, Result};
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
pub async fn get_user_session(
    session_id: String,
) -> Result<crate::types::UserSessionSS, ServerFnError> {
    use crate::types::AppState;
    use crate::types::UserSessionSS;
    use axum::Extension;
    use leptos::prelude::*;
    use leptos_axum::extract;
    use std::sync::Arc;

    let Extension(app_state): Extension<Arc<AppState>> = extract().await?;
    println!("user id: {:?}", session_id);
    let user_session_ss = match app_state
        .sesh
        .get::<UserSessionSS>(session_id.as_str())
        .await
    {
        Ok(us) => us,
        Err(e) => {
            leptos_axum::redirect("/signin?page=adduserdetails");
            return Err(ServerFnError::Registration(
                "user needs to re-sign in".to_string(),
            ));
        }
    };
    dbg!(&user_session_ss);
    let user_session_ss = match user_session_ss {
        Some(usss) => usss,
        None => {
            leptos_axum::redirect("/signin?page=adduserdetails");
            return Err(ServerFnError::Registration(
                "user needs to re-sign in".to_string(),
            ));
        }
    };
    return Ok(user_session_ss);
}
