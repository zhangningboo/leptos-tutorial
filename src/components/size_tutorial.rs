use leptos::prelude::*;

#[component]
pub fn SizeTutorial() -> impl IntoView {
    view! {
        <h1 class="flex justify-center text-xl font-bold bg-cyan-400 border-2 border-amber-600 rounded-md my-4">"Size Tutorial"</h1>
        // { md:block hidden } means show on medium screens and larger, hide on smaller screens
        <div class="md:block hidden">        
            <p class="text-cyan-400">"I appear on screen wider than 768px"</p>
        </div>
    }
}