use leptos::prelude::*;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

#[component]
pub fn ComplexForComponent() -> impl IntoView {
    let (data, set_data) = signal(vec![
        DatabaseEntry { key: "A".into(), value: RwSignal::new(1) },
        DatabaseEntry { key: "B".into(), value: RwSignal::new(2) },
        DatabaseEntry { key: "C".into(), value: RwSignal::new(3) },
        // RwSignal的替代版本: reactive_stores
    ]);

    view! {
        <button
            on:click=move |_| {
                set_data.update(|data| {
                    for row in data {
                        *row.value.write() += 1;
                    }
                });
                leptos::logging::log!("Data updated");
            }
        >
            "Update Values"
        </button>
        <For
            each=move || data.get()
            key=|state| state.key.clone()
            let(child)
        >
            <p>{child.value}</p>
        </For>
    }
}