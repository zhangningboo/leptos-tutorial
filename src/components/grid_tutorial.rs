use leptos::prelude::*;

#[component]
pub fn GridTutorial() -> impl IntoView {
    view! {
        <div>
            <h1 class="flex justify-center text-xl font-bold bg-violet-400 border-2 border-amber-600 rounded-md my-4">"Grid Tutorial"</h1>
            <div class="grid grid-cols-3 gap-4">
                <div class="h-16 w-16 rounded-full bg-blue-500"></div>
                <div class="h-16 w-16 rounded-full bg-orange-500"></div>
                <div class="h-16 w-16 rounded-full bg-green-500"></div>
            </div>

            <div class="grid grid-cols-3 gap-4 my-2">
                <div class="h-16 rounded-full bg-blue-500"></div>
                <div class="h-16 rounded-full bg-orange-500"></div>
                <div class="h-16 rounded-full bg-green-500"></div>
            </div>

            <a href="https://cssgridgarden.com/" class="text-blue-600 underline" target="_blank">"Learn more about CSS Grid"</a>
        </div>
    }
}