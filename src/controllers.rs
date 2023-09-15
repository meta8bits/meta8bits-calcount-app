
use super::{
    auth::Session, chrono_utils, client_events, components,
    components::Component, config, count_chat, errors::ServerError, metrics,
    models::AppState, stripe,
};
use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
    Form,
};
use chrono::{DateTime, Utc};
use futures::join;
use serde::Deserialize;
use sqlx::{query, query_as};

pub async fn root(
    State(AppState { db }): State<AppState>,
) -> Result<impl IntoResponse, ServerError> {
    let account_total =
        query!("select 1 count from users").fetch_all(&db).await?;
    let trial_accounts_remaining = config::MAX_ACCOUNT_LIMIT
        .checked_sub(account_total.len())
        .unwrap_or_default();
    Ok(components::Page {
        title: "Bean Count",
        children: &components::PageContainer {
            children: &components::Home {
                trial_accounts_remaining,
            },
        },
    }
    .render())
}

#[cfg(feature = "live_reload")]
#[derive(Deserialize)]
pub struct PongParams {
    pub poll_interval_secs: u64,
}
/// The client will reload when this HTTP long-polling route disconnects,
/// effectively implementing live-reloading.
#[axum_macros::debug_handler]
#[cfg(feature = "live_reload")]
pub async fn pong(
    axum::extract::Query(PongParams { poll_interval_secs }): axum::extract::Query<PongParams>,
) -> impl IntoResponse {
    tokio::time::sleep(std::time::Duration::from_secs(poll_interval_secs))
        .await;
    "pong"
}

#[cfg(not(feature = "live_reload"))]
pub async fn pong() -> impl IntoResponse {
    "pong"
}

/// You may be wondering why this sits on a separate response while the
/// tailwind styles are inlined into the page template and basically
/// hard-coded into every initial response. This is because the CSS is a
/// blocker for page rendering, so we want it right there in the initial
/// response. Meanwhile, it's fine for the browser to fetch and run HTMX
/// asynchronously since it doesn't really need to be on the page until the
/// first user interaction.
///
/// Additionally, our HTMX version does not change very often. We can exploit
/// browser cachine to mostly never need to serve this resource, making the
/// app more responsive and cutting down on overall bandwidth. That's also why
/// we have the HTMX version in the URL path -- because we need to bust the
/// browser cache every time we upgrade.
pub async fn get_htmx_js() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("text/javascript")
            .expect("We can insert text/javascript headers"),
    );
    headers.insert(
        "Cache-Control",
        HeaderValue::from_str("public, max-age=31536000")
            .expect("we can set cache control header"),
    );
    (headers, include_str!("./htmx-1.9.10.vendor.js"))
}

pub async fn get_favicon() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("image/x-icon")
            .expect("We can insert image/x-icon header"),
    );