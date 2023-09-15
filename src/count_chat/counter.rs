
//! The core calorie counting feature (models, components, and controllers
//! are colocated here).

use super::{llm_parse_response::ParserResult, openai::OpenAI};
use crate::{chrono_utils::is_before_today, client_events, config, prelude::*};
use axum::extract::Query;
use std::default::Default;

const MEAL_PAGE_SIZE: u8 = 50;

#[derive(Debug)]
pub struct Meal {
    id: i32,
    info: MealInfo,
}

#[derive(Debug, Deserialize)]
pub struct MealInfo {
    pub calories: i32,
    pub protein_grams: i32,
    pub carbohydrates_grams: i32,
    pub fat_grams: i32,
    pub meal_name: String,
    pub created_at: DateTime<Utc>,
}

pub struct Chat<'a> {
    pub meals: &'a Vec<Meal>,
    pub prompt: Option<&'a str>,
    pub user_timezone: Tz,
    pub next_page: i64,
    pub post_request_handler: Route,
}
impl Component for Chat<'_> {
    fn render(&self) -> String {
        let prev_meals = PreviousMeals {
            meals: self.meals,
            user_timezone: self.user_timezone,
            next_page: self.next_page,
        };
        ChatUI {
            post_request_handler: &self.post_request_handler,
            prefill_prompt: self.prompt,
            children: Some(&prev_meals),
        }
        .render()
    }
}

struct PreviousMeals<'a> {
    meals: &'a Vec<Meal>,
    user_timezone: Tz,
    next_page: i64,
}
impl Component for PreviousMeals<'_> {
    fn render(&self) -> String {
        let meals = MealSet {
            meals: &self.meals[..],
            user_timezone: self.user_timezone,
            next_page: self.next_page,
            show_ai_warning: false,
        }
        .render();
        let is_any_meal_during_today = self
            .meals
            .iter()
            .any(|m| !is_before_today(&m.info.created_at, self.user_timezone));
        let meal_header = if self.meals.is_empty() {
            ""
        } else if is_any_meal_during_today {
            // Pushing the top up by just 1px hides the text from revealing
            // itself behind the top of this sticky header as the user scrolls
            // through the container; weird browser behavior, weird fix.
            r#"<h2 class="
                    sticky
                    top-[-1px]
                    bg-zinc-50
                    dark:bg-slate-900
                    rounded
                    p-2
                    text-xl
                    font-bold
                ">
                    Today's Food
                </h2>"#
        } else {
            r#"<h2 class="
                    sticky
                    top-[-1px]
                    bg-sinc-50
                    dark:bg-slate-900
                    rounded
                    p-2
                    text-xl
                    font-bold
                ">
                    Previously Saved Items
                </h2>"#
        };
        let refresh_meals_href = format!("{}?page=0", Route::ListMeals);
        format!(
            r#"
            <div
                class="flex flex-col gap-2 md:max-h-[70vh] md:overflow-y-auto"
            >
                {meal_header}
                <div
                    hx-get="{refresh_meals_href}"
                    hx-swap="innerHTML"
                    hx-trigger="reload-meals from:body"
                    class="flex flex-col gap-2"
                >
                {meals}
                </div>
            </div>
            "#
        )
    }
}

pub struct ChatUI<'a> {
    pub post_request_handler: &'a Route,
    pub prefill_prompt: Option<&'a str>,
    /// If provided, these are inserted at the end of the chat container. This
    /// is used on the user home page for injecting the list of previous meals.
    pub children: Option<&'a dyn Component>,
}
impl Component for ChatUI<'_> {
    fn render(&self) -> String {
        let handler = &self.post_request_handler;
        let prompt = clean(self.prefill_prompt.unwrap_or_default());
        let children = self.children.map_or("".to_string(), |c| c.render());
        format!(
            r#"
            <div id="cal-chat-container" class="flex items-center justify-center">
                <div class="
                    bg-zinc-50
                    border-2
                    border-black
                    dark:bg-indigo-1000
                    md:dark:bg-blue-950
                    md:dark:border-white
                    m-2
                    md:p-4
                    p-2
                    rounded
                ">
                    <h1
                        class="
                            border-b-2
                            border-slate-600
                            mb-2
                            border-black
                            dark:border-slate-200
                            md:dark:border-black
                            serif
                            font-extrabold
                            text-3xl
                        ">
                            Calorie Chat
                        </h1>
                    <div class="md:flex md:gap-3">
                        <div>
                            <form
                                class="flex flex-col gap-2"
                                hx-post="{handler}"
                            >
                                <label for="chat">
                                    <h2
                                        class="text-xl bold"
                                    >Describe what you're eating</h2>
                                </label>
                                <input
                                    class="rounded"
                                    autocomplete="one-time-code"
                                    type="text"
                                    id="chat"
                                    name="chat"
                                    placeholder="I am eating..."
                                    value="{prompt}"
                                    required
                                />
                                <button class="
                                    bg-green-100
                                    dark:bg-green-800
                                    dark:hover:bg-green-700
                                    hover:bg-green-200
                                    p-2
                                    rounded
                                ">
                                    Count It
                                </button>
                            </form>
                        </div>
                        {children}
                    </div>
                </div>
            </div>
            "#
        )
    }
}

struct NewMealOptions<'a> {
    info: &'a MealInfo,
}
impl Component for NewMealOptions<'_> {
    fn render(&self) -> String {
        let retry_route = Route::ChatForm;
        let save_route = Route::SaveMeal;
        let calories = self.info.calories;
        let protein = self.info.protein_grams;
        let carbs = self.info.carbohydrates_grams;
        let fat = self.info.fat_grams;
        let created_at = self.info.created_at;
        let meal_name = clean(&self.info.meal_name);
        format!(
            r##"
            <form hx-post="{save_route}" hx-target="#cal-chat-container">
                <input type="hidden" value="{meal_name}" name="meal_name" />
                <input type="hidden" value="{calories}" name="calories" />
                <input type="hidden" value="{protein}" name="protein_grams" />
                <input type="hidden" value="{carbs}" name="carbohydrates_grams" />
                <input type="hidden" value="{fat}" name="fat_grams" />
                <input type="hidden" value="{created_at}" name="created_at" />
                <button
                    class="bg-blue-100 p-1 rounded shadow hover:bg-blue-200"
                >Save</button>