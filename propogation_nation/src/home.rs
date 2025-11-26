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

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <p>HOME PAGE ONCE LOGGED IN </p>
    }
}
