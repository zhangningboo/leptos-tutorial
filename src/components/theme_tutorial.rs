use leptos::prelude::*;

#[component]
pub fn ThemeTutorial() -> impl IntoView {
    view! {
        <h1 class="flex justify-center text-xl font-bold bg-cyan-400 border-2 border-amber-600 rounded-md my-4">"Dark Theme Tutorial"</h1>
        // { md:block hidden } means show on medium screens and larger, hide on smaller screens
        <div class="m-10 rounded-lg bg-white px-6 py-8 shadow-xl ring-1 ring-slate-900/5 dark:bg-black">        
            <h3 class="text-base font-medium tracking-tight text-slate-900 dark:text-white">"Writes Upside-Down"</h3>
            <p class="mt-2 text-sm text-slate-500 dark:text-blue-100">"The Zero Gravity Pen can be used to write in any orientation, including
            upside-down. It even works in outer space."</p>
            <button 
                on:click=move |_| {
                    let document = web_sys::window().unwrap().document().unwrap();
                    let html = document.document_element().unwrap();
                    if html.class_list().contains("dark") {
                        html.class_list().remove_1("dark").unwrap();
                    } else {
                        html.class_list().add_1("dark").unwrap();
                    }
                    println!("Toggled dark mode");
                }
                class="px-4 py-2 text-sm font-medium mt-8 text-blue-900 bg-blue-100 rounded-md">Toggle Dark Mode</button>
        </div>
    }
}