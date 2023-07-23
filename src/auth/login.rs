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
            <form class="flex flex-col gap-2 max-w-md" hx-post="{login_route}">
                <h1 class="text-xl">Login to Bean Count</h1>
                <label autocomplete="username" for="identifier">
                    Username or Email
                </label>
                <input
                    type="text"
                    id="identifier"
                    name="identifier"
                    autocomplete="username"
                />
                <label for="passwored">Password</label>
                <input
                    autocomplete="current-password"
                    type="password"
                    id="password"
                    name="password"
                    />
                <div class="flex gap-2">
                <button class="
                    block
                    bg-green-200
                    hover:bg-green-300
                    dark:bg-green-700
                    dark:hover:bg-green-600
                    dark:text-white
                    hover:shadow-none
                    p-1
                    rounded
                    shadow
                    transition
      