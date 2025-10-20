use leptos::prelude::*;

#[component]
pub fn FlexTutorial() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center">
            <div class="h-16 w-16 rounded-full bg-blue-500"></div>
            <div class="h-16 w-16 rounded-full bg-orange-500"></div>
            <div class="h-16 w-16 rounded-full bg-green-500"></div>
        </div>

        <a href="https://flexboxfroggy.com/" class="text-blue-600 underline" target="_blank">"Learn more about CSS Flexbox"</a>
    }
}
