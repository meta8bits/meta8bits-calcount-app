//! Glue which integrates [crate::pw], [crate::db_ops], and [crate::session].
//! Auth will authenticate users by fetching user info from the database and
//! authenticating a user with the provided credentials.

use super::{pw, Session};
use crate::{
    db_ops, db_ops::DbModel, models, preferences::get_user_preference,
};
use anyhow::{bail, Result};
use chrono::Utc;
use sqlx::{postgres::PgPool, query_as};

/// We are a bit losey goosey on the identifier for a better user experience.
/// I'm fairly convinced this is not a security issue. If we consider a
/// malicious user who creates an account where their username is someone else's
/// email, or their email is someone else's username, then they could certainly
/// get into a position where the `user` who is fetched by our database query
/// here is the target victim's user, and not the attacker's user. However,
/// the `tr