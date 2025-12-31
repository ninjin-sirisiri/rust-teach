use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="container pb-20">
            <section class="hero flex items-center justify-between text-left py-20">
                <div class="flex-1 pr-10">
                    <h1 class="text-6xl font-black leading-tight mb-6">"Rustを、" <br/> <span class="text-indigo-500">"体得する。"</span></h1>
                    <p class="text-xl text-slate-400 max-w-xl mb-8">"安全性、速度、そして並行性。現代のプログラミングに求められるすべてを、体系的に効率よく学習しましょう。"</p>
                    <div class="flex gap-4">
                        <A href="/lesson/intro" attr:class="px-8 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-bold transition-all transform">
                            "学習を始める"
                        </A>
                    </div>
                </div>
                <div class="flex-1 hidden md:block">
                    <div class="relative">
                        <img src="/hero.png" alt="Rust Visual" width="500" height="500" class="relative rounded-2xl border border-white/10 shadow-sm" />
                    </div>
                </div>
            </section>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                <div class="card p-8">
                    <h3 class="text-xl font-bold mb-4">"基礎から学ぶ"</h3>
                    <p class="text-slate-400">"Rustの哲学から基本的な構文まで、初心者が躓きやすいポイントを丁寧に解説します。"</p>
                </div>
                <div class="card p-8">
                    <h3 class="text-xl font-bold mb-4">"所有権の理解"</h3>
                    <p class="text-slate-400">"Rust最大の特徴である所有権。メモリ安全性を支えるこの仕組みを図解と共に学びます。"</p>
                </div>
                <div class="card p-8">
                    <h3 class="text-xl font-bold mb-4">"実践的な開発"</h3>
                    <p class="text-slate-400">"実際のプロダクト開発で使えるテクニックや、エラーハンドリングのベストプラクティスを紹介。"</p>
                </div>
            </div>
        </div>
    }
}
