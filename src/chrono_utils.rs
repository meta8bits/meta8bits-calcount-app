use chrono::prelude::*;
use chrono_tz::Tz;
use std::time::Duration;

/// Check if `time` is yesterday or before.
pub fn is_before_today(datetime: &DateTime<Utc>, user_timezone: Tz) -> bool {
    let local_dt = datetime.with