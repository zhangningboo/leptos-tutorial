use leptos::prelude::*;

#[component]
pub fn ProgressBar(
    #[prop(into)]
    progress: Signal<i32>,
    #[prop(default = 100)]
    max: u16,
) -> impl IntoView {
    view! {
        <progress
            max=max
            // hmm... where will we get this from?
            value=progress
        />
    }
}