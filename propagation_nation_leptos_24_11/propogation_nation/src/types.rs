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
