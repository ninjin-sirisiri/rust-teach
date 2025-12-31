use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Sidebar(is_open: ReadSignal<bool>, set_open: WriteSignal<bool>) -> impl IntoView {
    view! {
        <aside class=move || if is_open.get() { "open" } else { "" }>
            <div class="sidebar-label">"はじめに"</div>
            <div><A on:click=move |_| set_open.set(false) href="/" exact=true attr:class="nav-item">"ホーム"</A></div>

            <div class="sidebar-label">"基礎知識"</div>
            <div><A on:click=move |_| set_open.set(false) href="/lesson/intro" attr:class="nav-item">"Rustとは？"</A></div>
            <div><A on:click=move |_| set_open.set(false) href="/lesson/setup" attr:class="nav-item">"環境構築"</A></div>
            <div><A on:click=move |_| set_open.set(false) href="/lesson/hello" attr:class="nav-item">"Hello World"</A></div>

            <div class="sidebar-label">"基本構文"</div>
            <div><A on:click=move |_| set_open.set(false) href="/lesson/variables" attr:class="nav-item">"変数とデータ型"</A></div>
            <div><A on:click=move |_| set_open.set(false) href="/lesson/functions" attr:class="nav-item">"関数"</A></div>
            <div><A on:click=move |_| set_open.set(false) href="/lesson/control-flow" attr:class="nav-item">"制御フロー"</A></div>

            <div class="sidebar-label">"最重要概念"</div>
            <div><A on:click=move |_| set_open.set(false) href="/lesson/ownership" attr:class="nav-item">"所有権と借用"</A></div>
            <div><A on:click=move |_| set_open.set(false) href="/lesson/structs" attr:class="nav-item">"構造体"</A></div>
            <div><A on:click=move |_| set_open.set(false) href="/lesson/enums" attr:class="nav-item">"列挙型"</A></div>

            <div class="sidebar-label">"実践"</div>
            <div><A on:click=move |_| set_open.set(false) href="/lesson/collections" attr:class="nav-item">"コレクション"</A></div>
            <div><A on:click=move |_| set_open.set(false) href="/lesson/error-handling" attr:class="nav-item">"エラーハンドリング"</A></div>
        </aside>
    }
}
