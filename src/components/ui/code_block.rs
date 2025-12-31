use leptos::prelude::*;

#[component]
pub fn CodeBlock(code: String) -> impl IntoView {
    view! {
        <pre>
            <code>{code}</code>
        </pre>
    }
}
