//! Cookie-based session, secured by a HMAC signature.
use super::crypto;
use crate::{config, errors::ServerError, models::User, preferences};
use axum::headers::{HeaderMap, HeaderValue};
use base