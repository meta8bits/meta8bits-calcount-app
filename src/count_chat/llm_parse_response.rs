
//! An ad-hoc regex-y LLM response parser. Tries to tell the LLM to do better
//! next time an a resonably well-structured declarative way. Inspired by
//! https://www.youtube.com/watch?v=yj-wSRJwrrc.

use super::counter::MealInfo;
use regex::{Captures, Regex};

#[derive(Debug)]
pub struct FollowUp {
    pub parsing_error: String,
}

#[derive(Debug)]
pub enum ParserResult<T> {
    /// If we were able to parse a structured type from the LLM response, here
    /// it is!
    Ok(T),
    /// The follow-up string is intended to be sent back to the LLM for a
    /// retry, though it should also be a
    FollowUp(FollowUp),
}

impl MealInfo {
    pub fn parse(llm_text: &str, meal_name: &str) -> ParserResult<Self> {
        let calories_mo =
            Regex::new(r"(\d+)-?(\d+)? (of |in |total |the )*calories")
                .expect("cal regex is valid")
                .captures(llm_text);
        let protein_mo = Regex::new(
            r"(\d+)-?(\d+)?(g| grams) (of |in |total |the )*protein",
        )
        .expect("protein regex is valid")
        .captures(llm_text);
        let fat_mo =
            Regex::new(r"(\d+)-?(\d+)?(g| grams) (of |in |total |the )*fat")
                .expect("fat regex is valid")
                .captures(llm_text);
        let carbohydrates_mo = Regex::new(
            r"(\d+)-?(\d+)?(g| grams) (of |in |total |the )*(carbohydrates|carbs)",
        )
        .expect("carb regex is valid")
        .captures(llm_text);

        let calories = handle_capture(calories_mo.as_ref(), "calories");
        let protein = handle_capture(protein_mo.as_ref(), "protein (in grams)");
        let fat = handle_capture(fat_mo.as_ref(), "fat (in grams)");
        let carbohydrates = handle_capture(
            carbohydrates_mo.as_ref(),
            "carbohydrates (in grams)",
        );

        match (calories, protein, fat, carbohydrates) {
            (
                Ok(calories),
                Ok(protein_grams),
                Ok(fat_grams),
                Ok(carbohydrates_grams),
            ) => ParserResult::Ok(MealInfo {
                meal_name: meal_name.to_string(),
                calories,
                protein_grams,
                carbohydrates_grams,
                fat_grams,
                created_at: chrono::Utc::now(),
            }),
            (calories, protein, fat, carbs) => {
                ParserResult::FollowUp(FollowUp {
                    parsing_error: [calories, protein, fat, carbs].iter().fold(
                        String::new(),
                        |mut acc, res| {
                            if let Err(e) = res {
                                acc.push_str(e);
                            }
                            acc
                        },
                    ),
                })
            }
        }
    }
}

/// Returns the parsed i32 inside the match object, or a response message for
/// the LLM.
fn handle_capture<'a>(
    mo: Option<&'a Captures>,
    describe_to_llm: &'a str,
) -> Result<i32, String> {
    match mo {
        Some(v) => {
            let start = v.get(1);
            let end = v.get(2);
            match (start, end) {
                (Some(s), Some(e)) => {
                    let lower_end = s.as_str().parse::<i32>();
                    let upper_end = e.as_str().parse::<i32>();
                    match (lower_end, upper_end) {
                        (Ok(l), Ok(u)) => {
                            Ok((l + u) / 2)
                        }
                        _ => Err(format!(
                            "Could not parse the range of {describe_to_llm} ({}-{}) as a number.\n",