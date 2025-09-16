use leptos::prelude::*;


#[component]
pub fn Form() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());
    
    view! {
        <input
            class="border-4 border-indigo-500"
            type="text"
            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            on:input:target=move |ev| {  // input event每次输入都会触发
                // .value() returns the current value of an HTML input element
                set_name.set(ev.target().value());
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }

}