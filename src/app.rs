use crate::components::layout::header::Header;
use crate::components::layout::sidebar::Sidebar;
use crate::pages::home::Home;
use crate::pages::lesson::LessonPage;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    let (is_sidebar_open, set_sidebar_open) = signal(false);

    view! {
        <Router>
            <div class="min-h-screen flex flex-col">
                <Header is_open=is_sidebar_open set_open=set_sidebar_open />
                <div class="flex flex-1 pt-16">
                    <Sidebar is_open=is_sidebar_open set_open=set_sidebar_open />
                    <div
                        class=move || if is_sidebar_open.get() { "sidebar-overlay open" } else { "sidebar-overlay" }
                        on:click=move |_| set_sidebar_open.set(false)
                    ></div>
                    <main class="flex-1 p-8 ml-64 overflow-y-auto">
                        <Routes fallback=|| view! { "Not Found" }>
                            <Route path=path!("/") view=Home />
                            <Route path=path!("/lesson/:id") view=LessonPage />
                        </Routes>
                    </main>
                </div>
            </div>
        </Router>
    }
}
