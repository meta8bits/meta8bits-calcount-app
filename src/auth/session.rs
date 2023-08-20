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
/// session cookie by adding fields to this st