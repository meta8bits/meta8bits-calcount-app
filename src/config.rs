use std::time::Duration;

/// This is for all authentication sessions; users will need to log in again
/// every 7 days, since we basically have JWT authentication.
pub const SESSION_EXPIRY_TIME_DAYS: i64 = 7;

/// Password reset links will expire after 15 minutes.
pub const