use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(32);

    view! {
        <button
        on:click=move |_| set_count.set(4)>
        "meow: "
        {count}
        </button>
        <p>
        "double count: "
        {move || count.get() * 2}
        </p>
    }
}
