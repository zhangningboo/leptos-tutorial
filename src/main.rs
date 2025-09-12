use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| { *set_count.write() += 1; }
        >
            "Click me: "
            {count}
        </button>
        <p>"Double count: "</p>
        <p>
            "Reactively: {move || count.get() * 2}: " {move || count.get() * 2} 
        </p>
        <p>
            "Just rendered once: " {count.get() * 2}
        </p>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App)
}