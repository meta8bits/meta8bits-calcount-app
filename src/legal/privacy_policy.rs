use crate::prelude::*;

struct PrivacyPolicy;
impl Component for PrivacyPolicy {
    fn render(&self) -> String {
        r#"
        <div class="prose bg-slate-300 rounded p-2 md:p-4">
            <h1>Privacy Policy</h1>
            <p>
                