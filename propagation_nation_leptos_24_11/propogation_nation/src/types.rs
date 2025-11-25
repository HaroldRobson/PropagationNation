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
    pub fn to_UserSessionCS(self) -> UserSessionSS {
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
