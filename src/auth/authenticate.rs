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

/// We are a bit losey goosey on the identifier for a better u