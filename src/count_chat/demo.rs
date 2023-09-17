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
                class="bg-green-100 p-1 rounded shadow hover:bg-green-200"
                hx-target="#cal-chat-container"
                hx-get="{demo}"
            >Reset</button>
            "##
        )
    }
}

pub struct ChatDemo<'a> {
    pub prefill_prompt: Option<&'a str>,
}
impl Component for ChatDemo<'_> {
    fn render(&self) -> String {
        let options = [
            "5 second squeeze of honey",
            "hummus on brioche bread",
            "gigantic cheese burger",
            "half a dunkin boston cream",
            "3 handfuls of chex mix",
            "a greasy cheese burger",
            "a frozen chicken cutlet",
            "really big diner breakfast (traditional American)",
            "caesar salad & 10 stolen fries",
        ];
        let i = random::<usize>() % options.len();
        counter::ChatUI {
    