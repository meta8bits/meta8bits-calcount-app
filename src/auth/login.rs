use super::authenticate::authenticate;
use crate::{config, htmx, prelude::*};
use axum::{headers::HeaderValue, http::StatusCode};

pub struct LoginForm;
impl Component for LoginForm {
    fn render(&self) -> String {
        let login_route = Route::Login;
        let password_reset = Route::PasswordReset;
        format!(
            r#"
     