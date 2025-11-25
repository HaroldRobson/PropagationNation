#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::response::IntoResponse;
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sea_orm::{Database, DbErr};
    use serde::{Deserialize, Serialize};
    use ssr_tutorial::app::*;
    use ssr_tutorial::AppState;
    use time::Duration;
    use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

    #[cfg(feature = "ssr")]
    const COUNTER_KEY: &str = "counter";

    #[cfg(feature = "ssr")]
    #[derive(Default, Deserialize, Serialize)]
    struct Counter(usize);

    #[cfg(feature = "ssr")]
    async fn handler(session: Session) -> impl IntoResponse {
        let counter: Counter = session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();
        session.insert(COUNTER_KEY, counter.0 + 1).await.unwrap();
        format!("Current count: {}", counter.0)
    }

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let pool = Database::connect("postgresql://postgres@localhost:5432/leptos_dev")
        .await
        .unwrap();

    let app_state = AppState { pool: pool };

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(app_state.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
