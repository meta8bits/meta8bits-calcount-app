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
impl Default for UserPreference {
    fn default() -> Self {
        Self {
            timezone: Tz::UTC,
            caloric_intake_goal: None,
        }
    }
}

impl Component for UserPreference {
    fn render(&self) -> String {
        let tz = self.timezone;
        let goal = self
            .caloric_intake_goal
            .map_or("".to_string(), |g| g.to_string());
        let options = TZ_VARIANTS.iter().fold(String::