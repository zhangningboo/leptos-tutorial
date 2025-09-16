use leptos::prelude::*;

mod components;
mod chapter;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Counter {
  id: usize,
  count: RwSignal<i32>
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let html = "<p>This HTML will be injected.</p>";

    let values = vec![0, 1, 2];
    
    let length = 5;
    let _counters: Vec<RwSignal<Counter>> = (1..=length).map(|idx| RwSignal::new(Counter { id: idx, count: RwSignal::new(0) })).collect::<Vec<_>>();
    let (counters, _set_counters) = signal::<Vec<Counter>>(vec![Counter { id: 0, count: RwSignal::new(0) }, Counter { id: 0, count: RwSignal::new(0) }]);
    use crate::components::progress_bar::ProgressBar;
    use crate::chapter::chapter_3_4_for::ComplexForComponent;
    use crate::chapter::chapter_3_5_forms_and_inputs::Form;
    view! {
        <Form />
        <ComplexForComponent />

        <div inner_html=html/> 

        <ForEnumerate
            each=move || counters.get() // Same as <For/>
            key=|counter| counter.id    // Same as <For/>
            // Provides the index as a signal and the child T
            children={move |index: ReadSignal<usize>, counter: Counter| {
                view! {
                    <button
                        on:click=move |_| { *counter.count.write() += 1; }
                        class="px-3 py-1 bg-sky-600 hover:bg-sky-700 text-white rounded"
                    >{move || index.get()} ". Value: " {move || counter.count.get()}</button>
                    <br />
                }
            }}
        />

        <p>{values.clone()}</p>
        // or we can wrap them in <li>
        <ul>
            {values.clone().into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect::<Vec<_>>()}
        </ul>

        <ul>
            {values.clone().into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect_view()}
        </ul>

        <button
            on:click=move |_| { *set_count.write() += 1; }
            // class=("red", move || count.get() % 2 == 1)
            style="position: absolute;"
            style:left=move || format!("{}px", count.get() + 100)
            // style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", move || count.get().to_string())
            class="px-3 py-2 text-white bg-sky-400 hover:bg-sky-700"
            class=([ "rounded", ], move || count.get() % 2 == 1)
        >
            "Click me: "
            {count}
        </button>
        <br/>
        <ProgressBar attr:id="foo1" progress=count.clone() />
        <br/>
        <ProgressBar attr:id="foo2" progress=Signal::derive(double_count) />

        <p>"Double count: "</p>
        <p>"Reactively: {move || count.get() * 2}: " {double_count}</p>
        <p>"Just rendered once: " {count.get() * 2}</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}