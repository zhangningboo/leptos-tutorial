use leptos::prelude::*;

mod components;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="p-4 bg-">
            <components::flex_tutorial::FlexTutorial />
            <components::grid_tutorial::GridTutorial />
            <components::size_tutorial::SizeTutorial />
            <components::theme_tutorial::ThemeTutorial />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}