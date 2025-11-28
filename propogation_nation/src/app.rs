use crate::home::HomePage;
use crate::inputpages::auth::CreateInvalidatedUserView;
use crate::inputpages::auth::RequestPasswordResetView;
use crate::inputpages::auth::ResetPasswordView;
use crate::inputpages::auth::SignIn;
use crate::inputpages::auth::ValidateUserView;
use crate::inputpages::user::AddUserDetails;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

use crate::types::UserStateCS;
use codee::string::JsonSerdeCodec;
use leptos_use::use_cookie;
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/propogation_nation.css"/>

        // sets the document title
        <Title text="Welcome to Propogation Nation"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=LandingPage/>
                    <Route path=path!("/homepage") view = HomePage/>
                    <Route path=path!("/validate/:id") view=ValidateUserView/>
                    <Route path=path!("/reset/:id") view=ResetPasswordView/>
                    <Route path=path!("/signin") view=SignIn/>
                    <Route path=path!("/demo") view = Demo/>
                    <Route path=path!("/adduserdetails") view= AddUserDetails/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn LandingPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (state, set_state) = use_cookie::<UserStateCS, JsonSerdeCodec>("state");
    let signed_in = Resource::new(
        move || state.get(),
        |user_state| async move {
            match user_state {
                Some(us) => {
                    if let Some(id) = us.session_id {
                        check_session_exists(id).await.unwrap_or(false)
                    } else {
                        false
                    }
                }
                None => false,
            }
        },
    );

    let is_signed_in = move || signed_in.get().unwrap_or(false);

    view! {
        <Suspense>
        <h1>"Welcome to Propogation Nation"</h1>
        {move || if is_signed_in() {
            view!{<h4>"Signed In!" </h4>
            <p>
            <button on:click=move |_|  { set_state.set(None)}> "Sign Out"  </button>
            </p>
            }.into_any()
            }
        else { view! {<SignIn/>}.into_any()}}
        <h4>"Sign Up" </h4>
        <CreateInvalidatedUserView/>
        <h4>"Reset Password" </h4>
        <RequestPasswordResetView/>
        </Suspense>

    }
}

#[server]
pub async fn check_session_exists(id: String) -> Result<bool, ServerFnError> {
    use crate::types::AppState;
    use crate::types::UserSessionSS;
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
    match app_state.sesh.get::<UserSessionSS>(id.as_str()).await {
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(e) => Err(ServerFnError::ServerError(format!(
            "problen with tower store: {:?}",
            e
        ))),
    }
}

use leptos::prelude::*;
use leptos_use::{use_geolocation, UseGeolocationReturn};

#[component]
fn Demo() -> impl IntoView {
    let UseGeolocationReturn {
        coords,
        located_at,
        error,
        resume,
        pause,
    } = use_geolocation();

    view! {
        <pre lang="json">
            coords:
            {move || {
                if let Some(coords) = coords.get() {
                    format!(
                        r#"{{
        accuracy: {},
        latitude: {},
        longitude: {},
        altitude: {:?},
        altitude_accuracy: {:?},
        heading: {:?},
        speed: {:?},
    }}"#,
                        coords.accuracy(),
                        coords.latitude(),
                        coords.longitude(),
                        coords.altitude(),
                        coords.altitude_accuracy(),
                        coords.heading(),
                        coords.speed(),
                    )
                } else {
                    "None".to_string()
                }
            }}
            ,
            located_at: {located_at} ,
            error:
            {move || if let Some(error) = error.get() { error.message() } else { "None".to_string() }} ,
        </pre>
        <button on:click=move |_| pause()>"Pause watch"</button>
        <button on:click=move |_| resume()>"Resume watch"</button>
    }
}
