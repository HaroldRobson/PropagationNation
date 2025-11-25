use anyhow::{anyhow, Result};
use leptos::prelude::ServerFnError;
use leptos::prelude::*;
use leptos::server;
use leptos::task::spawn_local;
use serde::Serialize;

#[cfg(feature = "ssr")]
#[derive(Serialize)]
pub struct InvalidatedUser {
    pub email: Email,
    pub password: String,
    pub uuid: String,
}

#[derive(Serialize)]
pub struct Email(String);

pub trait Validate {
    fn is_valid(&self) -> bool;
}

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

    let invalidated_user =
        InvalidatedUser::new(email.clone(), password).map_err(to_server_error)?;
    let uuid_String = invalidated_user.uuid.clone();
    let uuid = uuid_String.as_str();

    println!("inserting user");
    app_state.sesh.insert(uuid, invalidated_user).await?;

    println!("emailing user");
    let email = Message::builder()
        .from(Mailbox::new(
            Some("NoBody".to_owned()),
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
        .subject(uuid)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy!"))
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
        <p>"Success!"</p>
    </Show>

        }
}
