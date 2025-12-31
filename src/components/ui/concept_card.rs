use leptos::prelude::*;

#[component]
pub fn ConceptCard(title: String, children: Children) -> impl IntoView {
    view! {
        <div class="my-8 p-6 rounded-2xl bg-gradient-to-br from-indigo-500/10 to-purple-500/10 border border-indigo-500/20 shadow-xl overflow-hidden relative group">
            <div class="absolute top-0 right-0 -mt-4 -mr-4 w-24 h-24 bg-indigo-500/10 rounded-full blur-2xl group-hover:bg-indigo-500/20 transition-all duration-500"></div>

            <div class="flex items-center gap-3 mb-4">
                <div class="p-2 rounded-lg bg-indigo-500/20 text-indigo-400">
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M12 2v8"/><path d="m4.93 10.93 1.41 1.41"/><path d="M2 18h2"/><path d="M20 18h2"/><path d="m19.07 10.93-1.41 1.41"/><path d="M22 22H2"/><path d="m8 22 4-10 4 10"/>
                    </svg>
                </div>
                <h3 class="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-indigo-400 to-purple-400">
                    {title}
                </h3>
            </div>

            <div class="text-slate-300 leading-relaxed italic whitespace-pre-wrap">
                {children()}
            </div>
        </div>
    }
}
