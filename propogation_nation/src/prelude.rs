pub mod client {
    pub use crate::types::UserStateCS;
    pub use anyhow::{anyhow, Result};
    pub use codee::string::JsonSerdeCodec;
    pub use leptos::prelude::*;
    pub use leptos::server_fn::codec::{MultipartData, MultipartFormData};
    pub use leptos::wasm_bindgen::JsCast;
    pub use leptos_router::components::*;
    pub use leptos_router::hooks::*;
    pub use leptos_router::hooks::{use_params, use_query};
    pub use leptos_router::params::Params;
    pub use leptos_router::*;
    pub use leptos_use::use_cookie;
    pub use web_sys::{FormData, HtmlFormElement, SubmitEvent};
}

#[cfg(feature = "ssr")]
pub mod server {
    pub use crate::types::{AppState, UserSessionSS};
    pub use crate::utils::*;
    pub use axum::Extension;
    pub use leptos::prelude::*;
    pub use leptos::server_fn::codec::{MultipartData, MultipartFormData};
    pub use leptos::wasm_bindgen::JsCast;
    pub use leptos_axum::extract;
    pub use sqlx::PgPool;
    pub use std::sync::Arc;
}
