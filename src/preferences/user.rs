//! User preferences

use crate::{
    components::{Page, PageContainer, Saved},
    prelude::*,
};
use axum::http::Method;
use chrono_tz::TZ_VARIANTS;
use serde::Serialize;
use std::default::Default;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct UserPreference {
    pub timezone: Tz,
    pub caloric_intake_goal: Option<i32>,
}
impl Default for UserPrefe