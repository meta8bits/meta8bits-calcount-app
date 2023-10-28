use crate::prelude::*;

struct PrivacyPolicy;
impl Component for PrivacyPolicy {
    fn render(&self) -> String {
        r#"
        <div class="prose bg-slate-300 rounded p-2 md:p-4">
            <h1>Privacy Policy</h1>
            <p>
                Information about all the food you eat is extremely personal
                and private data. It is extremely important to protect your
                privacy and data security. Your Bean Count data will never
                be shared with a 3rd party under and circumstances, which
                includes our email list as well as meal data you've entered
                into the app.
            <