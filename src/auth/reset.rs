use super::pw;
use crate::{
    auth,
    config::{DOMAIN, RESET_TOKEN_TIMEOUT_MINUTES},
    htmx,
    prelude::*,
    smtp::send_email,
};

struct ResetRequestForm;
impl Component for ResetRequestForm {
    fn render(&self) -> String {
        let reset_route = Route::PasswordReset;
        format!(
            r#"
            <form hx-post="{reset_route}" class="flex flex-col gap-2 max-w-prose p-2 sm:p-4 md:p-8">
                <h1 class="text-xl font-extrabold">Password Reset</h1>
      