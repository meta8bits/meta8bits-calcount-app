//! Cookie-based session, secured by a HMAC signature.
use super::crypto;
use crate::{config, errors::ServerError, models::User, preferences};
use axum::headers::{HeaderMap, HeaderValue};
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Days, Utc};
use chrono_tz::Tz;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// `Session` is signed and serialized into the `Cookie` header when a
/// [HeaderMap] is passed into the [Session::update_headers()] method. Thus,
/// it's easy to extend this framework to store more information in the secure
/// session cookie by adding fields to this struct. However, keep in mind that
/// since this struct is serialized into a HTTP header, it cannot get too large!
///
/// # Serialization & Deserialization Note
///
/// This struct does derive [Serialize] and [Deserialize]. Internally, these
/// are used to serialize the struct into JSON. Then, the
/// [Session::from_headers()] and [Session::update_headers()] methods perform
/// some additonal ad-hoc serialization and deserialization to grep the session
/// string out of the Cookie string (where it is prefixed by `session=`), and
/// also to convert to/from base64 encoding.
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub user: User,
    pub preferences: preferences::UserPreference,
    pub created_at: DateTime<Utc>,
}
impl Session {
    /// Parse the session from request headers, validating the cookie
    /// signature along the way. Returns the [None] variant if the session
    /// header is missing or invalid.
    pub fn from_headers(headers: &HeaderMap) -> Option<Self> {
        let cookie = headers.get("Cookie")?;
        let cookie = cookie.to_str().unwrap_or("");
        let re = Regex::new(r"session=(.*)").unwrap();
        let captures = re.captures(cookie)?;
        let token = &captures[1];
        let deserialize_result = Self::deserialize(token);

        if let Ok(session) = deserialize_result {
            Some(session)
        } else {
            None
        }
    }
    /// `err_msg` should identify which handler the error is coming from. Simply
    /// the name of the handler function is typically the best thing to put
    /// here.
    pub fn from_headers_err