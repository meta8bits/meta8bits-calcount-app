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
                <label for="email">Email Address</label>
                <p class="text-xs">
                    Input the email associated with your account
                </p>
                <input type="email" id="email" name="email" required />
                <button class="self-start dark:bg-green-700 dark:hover:bg-green-600 bg-green-100 hover:bg-green-200 rounded p-1">
                    Submit
                </button>
            </form>
            "#
        )
    }
}

struct ConfirmReset<'a> {
    email: &'a str,
}
impl Component for ConfirmReset<'_> {
    fn render(&self) -> String {
        let email = clean(self.email);
        let home = Route::Root;
        format!(
            r#"
            <div>
                <p>An password reset email was sent to {email} if an associated
                user exists.</p>
                <a class="link" href="{home}">Return to Home Page</a>
            </div>
            "#
        )
    }
}

pub async fn get_password_reset_request() -> String {
    Page {
        title: "Reset Password",
        children: &PageContainer {
            children: &ResetRequestForm {},
        },
    }
    .render()
}

#[derive(Deserialize)]
pub struct ResetPayload {
    email: String,
}

pub async fn handle_pw_reset_request(
    State(AppState { db }): State<AppState>,
    Form(ResetPayload { email }): Form<ResetPayload>,
) -> Result<impl IntoResponse, ServerError> {
    struct Qres {
        id: i32,
    }
    let uid = query_as!(Qres, "select id from users where email = $1", email)
        .fetch_optional(&db)
        .await?;
    if let Some(Qres { id }) = uid {
 