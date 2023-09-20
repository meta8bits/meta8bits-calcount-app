use crate::config;
use anyhow::{Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

pub struct OpenAI {
    client: Client,
    api_key: String,
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String