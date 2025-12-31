use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header(is_open: ReadSignal<bool>, set_open: WriteSignal<bool>) -> impl IntoView {
    view! {
        <header>
            <button
                class="menu-toggle"
                on:click=move |_| set_open.update(|v| *v = !*v)
            >
                {move || if is_open.get() {
                    view! {
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="18" y1="6" x2="6" y2="18"></line>
                            <line x1="6" y1="6" x2="18" y2="18"></line>
                        </svg>
                    }.into_any()
                } else {
                    view! {
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="3" y1="12" x2="21" y2="12"></line>
                            <line x1="3" y1="6" x2="21" y2="6"></line>
                            <line x1="3" y1="18" x2="21" y2="18"></line>
                        </svg>
                    }.into_any()
                }}
            </button>
            <A href="/" attr:class="logo">
                "Rust Teach"
            </A>
            <div style="flex: 1"></div>
            <nav class="flex items-center">
                <a href="https://github.com/rust-lang/rust" target="_blank" class="nav-item">
                    "GitHub"
                </a>
            </nav>
        </header>
    }
}
