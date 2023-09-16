use super::{counter, llm_parse_response, openai};
use crate::{config, prelude::*};
use rand::random;

struct DemoMealOptions;
impl Component for DemoMealOptions {
    fn render(&self) -> String {
        let demo = Route::ChatDemo;
        format!(
            r##"
            <button
                class="bg-green-100 p-1 r