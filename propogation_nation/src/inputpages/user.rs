// theres a lot of user stuff in auth for onboarding, but the idea is they can optionally provide
// more details for a social media style account, and people can view their profile.
#[cfg(feature = "ssr")]
use crate::types::UserSessionSS;
use anyhow::{anyhow, Result};
#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;
use leptos::prelude::*;
use leptos::server;
use leptos::server_fn::codec::{MultipartData, MultipartFormData};
#[cfg(feature = "ssr")]
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::Params;
use leptos_router::hooks::{use_params, use_query};
use leptos_router::params::Params;
#[cfg(feature = "ssr")]
use serde::Deserialize;
#[cfg(feature = "ssr")]
use serde::Serialize;
#[cfg(feature = "ssr")]
use tower_sessions::session;
#[cfg(feature = "ssr")]
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

#[component]
pub fn AddUserDetails() -> impl IntoView {
    use crate::types::UserStateCS;
    use codee::string::JsonSerdeCodec;
    use leptos_use::use_cookie;
    use web_sys::{FormData, HtmlFormElement, SubmitEvent};
    let (state, set_state) = use_cookie::<UserStateCS, JsonSerdeCodec>("state");
    let session_id = move || match state.get() {
        None => "".to_string(),
        Some(uscs) => uscs.session_id.unwrap_or("".to_string()),
    };
    let (location, set_location) = signal("".to_string());
    let upload_action = Action::new_local(move |data: &FormData| {
        let dat = data.clone().into();
        // `MultipartData` implements `From<FormData>`
        async move { add_user_details(dat).await }
    });

    view! {
        <fieldset>
        <label>
        "enter your borough"
        <input type="text"

        placeholder="Hackney".to_string()
        on:input:target=move |ev| {
                // .value() returns the current value of an HTML input element
                set_location.set(ev.target().value());
            }
        prop:value=location
        />
        </label>
        <p>Upload a selfie!</p>
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();
            let _ = form_data.append_with_str("location", location.get().as_str());
            let _ = form_data.append_with_str("session_id", session_id().as_str());
            upload_action.dispatch_local(form_data);
        }>
            <input type="file" name="file_to_upload" />
            <input type="submit" />
        </form>
        <p>
            {move || {
                if upload_action.input().read().is_none() && upload_action.value().read().is_none()
                {
                    "Upload a file.".to_string()
                } else if upload_action.pending().get() {
                    "Uploading...".to_string()
                } else if let Some(Ok(val)) = upload_action.value().get() {
                    "Success!".to_string()
                } else {
                    format!("{:?}", upload_action.value().get())
                }
            }}

        </p>

        </fieldset>
    }
}

#[server(
        input = MultipartFormData,
    )]
pub async fn add_user_details(data: MultipartData) -> Result<String, ServerFnError> {
    use crate::types::AppState;
    use axum::Extension;
    use leptos_axum::extract;
    use std::sync::Arc;
    use uuid::Uuid;
    let mut data = data.into_inner().unwrap();
    let mut all_bytes: Vec<u8> = Vec::new();
    let mut location = "".to_string();
    let mut session_id = "".to_string();
    while let Ok(Some(mut field)) = data.next_field().await {
        println!("\n[NEXT FIELD]\n");
        let name = field.name().unwrap_or_default();
        println!("  [NAME] {:?}", &name);
        match name {
            "file_to_upload" => {
                while let Ok(Some(chunk)) = field.chunk().await {
                    all_bytes.extend(&chunk);
                }
            }
            "location" => {
                location = field.text().await?;
            }
            "session_id" => {
                session_id = field.text().await?;
            }
            _ => {}
        }
    }
    use crate::types::UserSessionSS;
    use crate::utils;
    let user_session_ss: UserSessionSS = match utils::get_user_session(session_id).await {
        Ok(usss) => usss,
        Err(e) => {
            leptos_axum::redirect("/signin?page=adduserdetails");
            return Ok("User needs to re-sign in".to_string());
        }
    };

    use file_type::FileType;
    let filetype = FileType::from_bytes(&all_bytes)
        .extensions()
        .first()
        .unwrap_or(&"");
    let allowed_file_types = ["png", "jpeg", "jpg"];
    if !allowed_file_types.contains(filetype) {
        return Err(ServerFnError::new("Invalid File Type!"));
    }
    let file_id = Uuid::new_v4().to_string();
    let file_name = format!("./uploads/{}.{}", file_id, filetype);
    dbg!(&file_name);
    use std::fs;
    fs::write(file_name.as_str(), all_bytes)?;

    let Extension(app_state): Extension<Arc<AppState>> = extract().await?;
    let pool = &app_state.pool;
    sqlx::query!(
        "INSERT INTO user_details (id, photo_url, location) VALUES ($1, $2, $3)",
        user_session_ss.id,
        file_name.as_str(),
        location.as_str()
    )
    .execute(pool)
    .await?;
    leptos_axum::redirect("/homepage");

    Ok(file_name)
}
