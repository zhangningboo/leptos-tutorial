use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let html = "<p>This HTML will be injected.</p>";

    view! {
        <div inner_html=html/>
        
        <button
            on:click=move |_| { *set_count.write() += 1; }
            // class=("red", move || count.get() % 2 == 1)
            style="position: absolute;"
            style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", move || count.get().to_string())
            class=(["button-20", "rounded"], move || count.get() % 2 == 1)
        >
            "Click me: "
            {count}
        </button>
        <progress
            max="50"
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value=double_count
        />
        <p>"Double count: "</p>
        <p>
            "Reactively: {move || count.get() * 2}: " {double_count} 
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