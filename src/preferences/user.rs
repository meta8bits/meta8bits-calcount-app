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
        let options = TZ_VARIANTS.iter().fold(String::new(), |mut acc, tz_choice| {
            let selected = if *tz_choice == tz {
                "selected"
            } else {
                ""
            };
            acc.push_str(&format!(r#"<option {selected} value="{tz_choice}">{tz_choice}</option>\n"#));
            acc
        });
        let self_url = Route::UserPreference;
        let home = Route::UserHome;
        format!(
            r#"
            <div class="flex flex-col items-center justify-center max-w-prose">
                <form
                    hx-post="{self_url}"
                    class="p-4 bg-slate-200 text-black rounded w-prose flex
                    flex-col gap-2"
                >
                    <h1 class="text-2xl font-extrabold">User Preferences</h1>
                    <label for="timezone">Timezone</label>
                    <select
                        id="timezone"
                        name="timezone"
                    >{options}</select>
                    <label for="caloric_intake_goal">Caloric Intake Goal</label>
                    <p class="text-sm">
                        This should be based on your Total Daily Energy
                        Expenditure (TDEE), and your goals for weight loss,
                        maintainance, or gain. Use an online resource like
                        <a class="link" href="https://tdeecalculator.net/">the
                        TDEE calculator</a> to calculate the perfect calorie
                        goal for you.
                    </p>
                    <input
                        type="number"
                        step="100"
                        value