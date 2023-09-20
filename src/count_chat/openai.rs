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
    model: String,
    messages: Vec<ChatCompletionMessage>,
}

#[derive(Serialize)]
struct ChatCompletionMessage {
    role: MessageRole,
    content: String,
}

#[derive(Serialize)]
#[allow(non_camel_case_types)]
enum MessageRole {
    user,
    system,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatCompletionResponseMessage>,
    usage: Usage,
}

#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completio