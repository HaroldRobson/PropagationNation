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

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct InvalidatedUser {
    pub email: Email,
    pub password: String,
    pub uuid: String,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Email(String);

pub trait Validate {
    fn is_valid(&self) -> bool;
}

#[cfg(feature = "ssr")]
impl Validate for Email {
    fn is_valid(&self) -> bool {
        self.0.contains('@')
            && self.0.contains('.')
            && self.0.len() > 3
            && !self.0.starts_with('@')
            && !self.0.ends_with('@')
    }
}

#[cfg(feature = "ssr")]
impl InvalidatedUser {
    pub fn new(email: String, password: String) -> Result<Self> {
        use uuid::Uuid;
        let email = Email(email);
        if email.is_valid() {
            let uuid = Uuid::new_v4().to_string();
            let invalidated_user = InvalidatedUser {
                email: email,
                password: password,
                uuid: uuid,
            };
            return Ok(invalidated_user);
        }
        return Err(anyhow!("invalid email".to_string()));
    }
}

fn to_server_error<E: ToString>(e: E) -> ServerFnError {
    ServerFnError::ServerError(e.to_string())
}

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ValidatedUser {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[server]
pub async fn create_invalidated_user(email: String, password: String) -> Result<(), ServerFnError> {
    use crate::types::AppState;
    use axum::Extension;
    use leptos::prelude::*;
    use leptos_axum::extract;
    use lettre::message::{header::ContentType, Mailbox};
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    use std::sync::Arc;

    println!("creationg user");
    let Extension(app_state): Extension<Arc<AppState>> = extract().await?;
    match sqlx::query_scalar!(
        "SELECT NOT EXISTS(SELECT 1 FROM users WHERE email = $1)",
        email.clone()
    )
    .fetch_one(&app_state.pool)
    .await?
    {
        Some(true) => {}
        None => {
            return Err(ServerFnError::MiddlewareError(
                "User already exists".to_owned(),
            ))
        }

        Some(false) => {
            return Err(ServerFnError::MiddlewareError(
                "User already exists".to_owned(),
            ))
        }
    }

    let invalidated_user =
        InvalidatedUser::new(email.clone(), password).map_err(to_server_error)?;
    let uuid_String = invalidated_user.uuid.clone();
    let uuid = uuid_String.as_str();

    println!("inserting user");
    app_state.sesh.insert(uuid, invalidated_user).await?;

    let link = format!("localhost:3000/validate/{:}", uuid_String);

    println!("emailing user");
    let email = Message::builder()
        .from(Mailbox::new(
            Some("propogation_nation".to_owned()),
            "nobody@domain.tld".parse().unwrap(),
        ))
        .reply_to(Mailbox::new(
            Some("Yuin".to_owned()),
            "yuin@domain.tld".parse().unwrap(),
        ))
        .to(Mailbox::new(
            Some("meowie".to_owned()),
            email.parse().unwrap(),
        ))
        .subject("your link to sign up".to_owned())
        .header(ContentType::TEXT_PLAIN)
        .body(link)
        .unwrap();
    app_state.mailer.send(&email)?;
    Ok(())
}

#[component]
pub fn CreateInvalidatedUserView() -> impl IntoView {
    let (email, set_email) = signal("enter email".to_string());
    let (password, set_password) = signal("enter password".to_string());
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(false);
    let create_user_action = Action::new(|(e, p): &(String, String)| {
        let e = e.clone();
        let p = p.clone();
        async move { create_invalidated_user(e.to_owned(), p.to_owned()).await }
    });

    Effect::new(move |_| {
        if let Some(result) = create_user_action.value().get() {
            match result {
                Ok(_) => {
                    leptos::logging::log!("User created successfully!");
                    set_success.set(true);
                    set_error.set(None);
                }
                Err(e) => {
                    leptos::logging::log!("Error creating user: {}", e);
                    set_error.set(Some(e.to_string()));
                    set_success.set(false);
                }
            }
        }
    });

    view! {
            <input type="text"
            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            on:input:target=move |ev| {
                // .value() returns the current value of an HTML input element
                set_email.set(ev.target().value());
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=email
        />
        <input type="text"
            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            on:input:target=move |ev| {
                // .value() returns the current value of an HTML input element
                set_password.set(ev.target().value());
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=password
        />
        <button on:click= move |_| {
            leptos::logging::log!("Button clicked!");
            create_user_action.dispatch((email.get(), password.get()));
        }>"Register" </button>
    <Show
        when=move || success.get()
        fallback=move || view! {
            <Show
                when=move || error.get().is_some()
                fallback=|| view! { <p></p> }
            >
                {move || view! { <p>"Error: " {error.get().unwrap()}</p> }}
            </Show>
        }
    >
        <p>"Success! Click on the link in your inbox"</p>
    </Show>

        }
}

#[derive(Params, PartialEq)]
struct UuidParams {
    id: Option<String>,
}

#[server]
pub async fn validate_and_create_user(
    id: String,
    name: String,
    description: String,
    lat: f32,
    lon: f32,
) -> Result<(), ServerFnError> {
    use crate::types::AppState;
    use axum::Extension;
    use leptos::prelude::*;
    use leptos_axum::extract;
    use lettre::message::{header::ContentType, Mailbox};
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    use sqlx::postgres::PgPool;
    use std::sync::Arc;

    let Extension(app_state): Extension<Arc<AppState>> = extract().await?;
    println!("user id: {:?}", id);
    let invalidated_user_optional = match app_state.sesh.get::<InvalidatedUser>(id.as_str()).await {
        Ok(ivu) => ivu,
        Err(e) => {
            return Err(ServerFnError::Registration(
                "error registering - sign up again".to_string(),
            ));
        }
    };
    dbg!(&invalidated_user_optional);
    let invalidated_user = match invalidated_user_optional {
        Some(ivu) => ivu,
        None => {
            return Err(ServerFnError::Registration(
                "error registering - sign up again".to_string(),
            ));
        }
    };
    let pool = &app_state.pool;
    sqlx::query!("INSERT INTO users (email, password, name, description, lat, lon) VALUES ($1, $2, $3, $4, $5, $6)",
    invalidated_user.email.0, invalidated_user.password, name, description, lat, lon)
        .execute(pool).await?;
    app_state
        .sesh
        .remove::<InvalidatedUser>(id.as_str())
        .await?;

    Ok(())
}

#[component]
pub fn ValidateUserView() -> impl IntoView {
    let params = use_params::<UuidParams>();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
            .unwrap_or_default()
    };
    let favorite_color = RwSignal::new("red".to_string());
    signal("I have been propogating for x years....".to_string());
    let (lat, set_lat) = signal::<f32>(1f32);
    let (lon, set_lon) = signal::<f32>(1f32);
    let spam_me = RwSignal::new(true);
    let create_user = ServerAction::<ValidateAndCreateUser>::new();

    view! {

        <fieldset>
            <legend>"User Sign Up Form"</legend>
        <ActionForm action=create_user>
        <input type="hidden" name= "id" value=id/>
        <fieldset>
            <legend>"Your Name"</legend>
        <input type="text"
        name="name"
        />
        </fieldset>
        <fieldset>
            <legend>"About You"</legend>
        <textarea
            name="description"
            rows="6"
            cols="90"
            placeholder="I have been propogating for x years..."
        />
        </fieldset>
        <fieldset>
            <legend>"latitude and longitude"</legend>
        <input type="number"
            step="0.000001"
            name="lat"
       />
        <input type="number"
            step="0.000001"
            name="lon"
       />

        </fieldset>


        <label>
            "I Agree to receive email Updates"
            <input type="checkbox"
            name="spam_me"
            />
        </label>
      <button type="submit">"Submit"</button>
        </ActionForm>
        <Show when=move || create_user.value().get().is_some()>
        view! {
        { move || {
                create_user.value().get().map(|result| {
                    match result {
            Ok(_) => view! {
                <div class="success">
                    <p>"User Created"</p>
                </div>
            }.into_any(),
            Err(e) => view! {
                <div class="error">
                    <p> "Error: " {e.to_string()}</p>
                    </div>
            }.into_any(),
        }
                  })}}
        }
        </Show>
        </fieldset>
    }
}

#[server]
pub async fn sign_in(email: String, password: String) -> Result<String, ServerFnError> {
    use crate::types::AppState;
    use crate::types::UserDetails;
    use crate::types::UserSessionSS;
    use axum::Extension;
    use leptos::prelude::*;
    use leptos_axum::extract;
    use lettre::message::{header::ContentType, Mailbox};
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    use sqlx::postgres::PgPool;
    use std::sync::Arc;
    use uuid::Uuid;

    let Extension(app_state): Extension<Arc<AppState>> = extract().await?;
    let pool = &app_state.pool;

    let user_password = sqlx::query_scalar!("SELECT password FROM users WHERE email = $1", email)
        .fetch_one(pool)
        .await?;

    match user_password == password {
        true => println!("valid password"),
        false => {
            return Err(ServerFnError::MiddlewareError(
                "password does not match".to_owned(),
            ))
        }
    }

    let user_details = sqlx::query_as!(
        UserDetails,
        "SELECT id, email, name FROM users WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await?;
    let id = Uuid::new_v4().to_string();

    let user_session_ss = user_details.to_UserSessionCS();

    app_state.sesh.insert(id.as_str(), user_session_ss).await?;

    Ok(id)
}

use crate::types::UserStateCS;
use codee::string::JsonSerdeCodec;
use leptos_use::use_cookie;
#[component]
pub fn SignIn() -> impl IntoView {
    let (email, set_email) = signal("enter email".to_string());
    let (password, set_password) = signal("enter password".to_string());
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(false);
    let sign_in_action = Action::new(|(e, p): &(String, String)| {
        let e = e.clone();
        let p = p.clone();
        async move { sign_in(e.to_owned(), p.to_owned()).await }
    });
    let (state, set_state) = use_cookie::<UserStateCS, JsonSerdeCodec>("state");
    Effect::new(move |_| {
        if let Some(result) = sign_in_action.value().get() {
            match result {
                Ok(session_id) => {
                    leptos::logging::log!("User signed in successfully!");
                    set_success.set(true);
                    set_error.set(None);
                    let state_cs = UserStateCS::new(session_id);
                    set_state.set(Some(state_cs));
                }
                Err(e) => {
                    leptos::logging::log!("Error creating user: {}", e);
                    set_error.set(Some(e.to_string()));
                    set_success.set(false);
                }
            }
        }
    });

    view! {
        <h3> "Sign In" </h3>
            <input type="text"
            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            on:input:target=move |ev| {
                // .value() returns the current value of an HTML input element
                set_email.set(ev.target().value());
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=email
        />
        <input type="text"
            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            on:input:target=move |ev| {
                // .value() returns the current value of an HTML input element
                set_password.set(ev.target().value());
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=password
        />
        <button on:click= move |_| {
            leptos::logging::log!("Button clicked!");
            sign_in_action.dispatch((email.get(), password.get()));
        }>"Sign In" </button>
    <Show
        when=move || success.get()
        fallback=move || view! {
            <Show
                when=move || error.get().is_some()
                fallback=|| view! { <p></p> }
            >
                {move || view! { <p>"Error: " {error.get().unwrap()}</p> }}
            </Show>
        }
    >
        <p>"Success!"</p>
    </Show>

        }
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct PasswordReset {
    pub email: String,
    pub uuid: String,
}

#[server]
pub async fn request_password_reset(email: String) -> Result<(), ServerFnError> {
    use crate::types::AppState;
    use axum::Extension;
    use leptos::prelude::*;
    use leptos_axum::extract;
    use lettre::message::{header::ContentType, Mailbox};
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    use std::sync::Arc;
    use uuid::Uuid;

    println!("creationg user reset link");
    let Extension(app_state): Extension<Arc<AppState>> = extract().await?;
    match sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
        email.clone()
    )
    .fetch_one(&app_state.pool)
    .await?
    {
        Some(true) => {}
        None => {
            return Err(ServerFnError::MiddlewareError(
                "No user found for email".to_owned(),
            ))
        }

        Some(false) => {
            return Err(ServerFnError::MiddlewareError(
                "No user found for email".to_owned(),
            ))
        }
    }
    let uuid_String = Uuid::new_v4().to_string();

    let password_reset = PasswordReset {
        email: email.clone(),
        uuid: uuid_String.clone(),
    };
    let uuid = uuid_String.as_str();

    println!("inserting user");
    app_state.sesh.insert(uuid, password_reset).await?;

    let link = format!("localhost:3000/reset/{:}", uuid_String);

    println!("emailing user");
    let email = Message::builder()
        .from(Mailbox::new(
            Some("propogation_nation".to_owned()),
            "nobody@domain.tld".parse().unwrap(),
        ))
        .reply_to(Mailbox::new(
            Some("Yuin".to_owned()),
            "yuin@domain.tld".parse().unwrap(),
        ))
        .to(Mailbox::new(
            Some("meowie".to_owned()),
            email.parse().unwrap(),
        ))
        .subject("your link to reset password".to_owned())
        .header(ContentType::TEXT_PLAIN)
        .body(link)
        .unwrap();
    app_state.mailer.send(&email)?;
    Ok(())
}

#[component]
pub fn RequestPasswordResetView() -> impl IntoView {
    let request_reset = ServerAction::<RequestPasswordReset>::new();
    let value = request_reset.value();
    // check if the server has returned an error
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));
    view! {
    <ActionForm action = request_reset>
        <label>
        "Request Password Reset"
        <input type="text" name="email"/>
        </label>
        <button type="submit"> "Request" </button>
    </ActionForm>
    {move || match value.get() {
        Some(Ok(_)) => view! {<p> "Success! Click on the link in your inbox" </p>}.into_any(),
        Some(Err(e)) => view! {<p> "Error: " {e.to_string()}</p>}.into_any(),
        None => view! {}.into_any(),
    }}
    }
}

#[derive(Params, PartialEq)]
struct UuidResetParams {
    id: Option<String>,
}

#[server]
pub async fn reset_password(id: String, new_password: String) -> Result<(), ServerFnError> {
    use crate::types::AppState;
    use axum::Extension;
    use leptos::prelude::*;
    use leptos_axum::extract;
    use lettre::message::{header::ContentType, Mailbox};
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    use sqlx::postgres::PgPool;
    use std::sync::Arc;

    let Extension(app_state): Extension<Arc<AppState>> = extract().await?;
    println!("user id: {:?}", id);
    let password_reset_data_optional = match app_state.sesh.get::<PasswordReset>(id.as_str()).await
    {
        Ok(prd) => prd,
        Err(e) => {
            return Err(ServerFnError::Registration(
                "error resetting password - try again".to_string(),
            ));
        }
    };
    let password_reset_data = match password_reset_data_optional {
        Some(prd) => prd,
        None => {
            return Err(ServerFnError::Registration(
                "error resetting password - try again".to_string(),
            ));
        }
    };
    let pool = &app_state.pool;
    sqlx::query!(
        "UPDATE users SET password = $1 WHERE email = ($2)",
        new_password,
        password_reset_data.email
    )
    .execute(pool)
    .await?;
    app_state.sesh.remove::<PasswordReset>(id.as_str()).await?;

    Ok(())
}

#[component]
pub fn ResetPasswordView() -> impl IntoView {
    let params = use_params::<UuidResetParams>();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
            .unwrap_or_default()
    };
    let reset_password_action = ServerAction::<ResetPassword>::new();

    view! {

        <fieldset>
            <legend>"Password Reset Form"</legend>
        <ActionForm action=reset_password_action>
        <input type="hidden" name= "id" value=id/>
        <fieldset>
            <legend>"New Password"</legend>
        <input type="text"
        name="new_password"
        />
        </fieldset>
     <button type="submit">"Submit"</button>
        </ActionForm>
        <Show when=move || reset_password_action.value().get().is_some()>
        view! {
        { move || {
                reset_password_action.value().get().map(|result| {
                    match result {
            Ok(_) => view! {
                <div class="success">
                    <p>"Password Updated"</p>
                </div>
            }.into_any(),
            Err(e) => view! {
                <div class="error">
                    <p> "Error: " {e.to_string()}</p>
                    </div>
            }.into_any(),
        }
                  })}}
        }
        </Show>
        </fieldset>
    }
}
