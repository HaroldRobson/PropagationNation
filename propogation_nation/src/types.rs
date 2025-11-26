#[cfg(feature = "ssr")]
use lettre::SmtpTransport;
#[cfg(feature = "ssr")]
use sqlx::postgres::PgPool;
#[cfg(feature = "ssr")]
use tower_sessions::Session;

#[derive(Clone, Debug)]
#[cfg(feature = "ssr")]
pub struct AppState {
    pub pool: PgPool,
    pub sesh: Session,
    pub mailer: SmtpTransport,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg(feature = "ssr")]
pub struct UserSessionSS {
    // server one
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[cfg(feature = "ssr")]
impl UserSessionSS {
    pub fn new(id: i32, email: String, name: String) -> Self {
        UserSessionSS {
            id: id,
            email: email,
            name: name,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[cfg(feature = "ssr")]
pub struct UserDetails {
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[cfg(feature = "ssr")]
impl UserDetails {
    pub fn to_UserSessionSS(self) -> UserSessionSS {
        UserSessionSS::new(self.id, self.email, self.name)
    }
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserStateCS {
    pub session_id: Option<String>,
}

impl UserStateCS {
    pub fn new(id: String) -> Self {
        Self {
            session_id: Some(id),
        }
    }
}

use std::default::Default;

impl Default for UserStateCS {
    fn default() -> Self {
        return Self { session_id: None };
    }
}

use time::OffsetDateTime;
// only FromRow if server side (cfg_attr is such a cool macro!):
use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
struct Plant {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: time::OffsetDateTime,
    pub location: String,
    pub user_id: i32, // owner
    pub picture_urls: Vec<String>,
    #[cfg(feature = "ssr")] // only let the server be able to view exact location
    pub lat: f32,
    #[cfg(feature = "ssr")]
    pub lon: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserDetailsFull {
    pub id: i32,
    pub email: Option<String>, // depends on email_public in users
    pub name: String,
    pub picture_url: Option<String>,
    pub location: String,
    pub description: String,
    pub created_at: time::OffsetDateTime,
}
