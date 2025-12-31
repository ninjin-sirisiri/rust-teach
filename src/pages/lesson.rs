use crate::components::ui::code_block::CodeBlock;
use crate::components::ui::concept_card::ConceptCard;
use crate::lessons::{get_lesson_order, get_lessons};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

#[component]
pub fn LessonPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap_or_default());
    let lessons = get_lessons();

    let order = get_lesson_order();

    let lesson = move || {
        let lesson_id = id();
        lessons.get(&lesson_id).map(|l| {
            let idx = order.iter().position(|x| x == &lesson_id);

            let prev = idx.and_then(|i| {
                if i > 0 {
                    let id = order[i - 1];
                    lessons.get(id).map(|l| (id.to_string(), l.title.clone()))
                } else {
                    None
                }
            });

            let next = idx.and_then(|i| {
                if i < order.len() - 1 {
                    let id = order[i + 1];
                    lessons.get(id).map(|l| (id.to_string(), l.title.clone()))
                } else {
                    None
                }
            });

            (
                l.title.clone(),
                l.content.clone(),
                l.code_examples.clone(),
                l.learning_points.clone(),
                prev,
                next,
            )
        })
    };

    view! {
        <div class="container animate-in py-10 max-w-4xl mx-auto">
            {move || match lesson() {
                Some((title, content, examples, learning_points, prev, next)) => {
                    let content_owned = content.clone();

                    view! {
                        <div class="lesson-content">
                            <h1 class="text-4xl font-bold mb-8 bg-clip-text text-transparent bg-gradient-to-r from-white to-slate-400">
                                {title}
                            </h1>

                            <div class="space-y-6">
                                {
                                    let mut sections = Vec::new();
                                    let mut current_chunk = Vec::new();
                                    let mut in_code_block = false;

                                    let lines: Vec<&str> = content_owned.lines().collect();

                                    for line in lines {
                                        if line.trim().starts_with("```") {
                                            if in_code_block {
                                                // Code block finishes
                                                current_chunk.push(line);
                                                let code_block = current_chunk.join("\n");
                                                sections.push((true, code_block));
                                                current_chunk.clear();
                                                in_code_block = false;
                                            } else {
                                                // Code block starts
                                                // Flush previous text chunk if any
                                                if !current_chunk.is_empty() {
                                                    let text_chunk = current_chunk.join("\n");
                                                    sections.push((false, text_chunk));
                                                    current_chunk.clear();
                                                }
                                                current_chunk.push(line);
                                                in_code_block = true;
                                            }
                                        } else {
                                            current_chunk.push(line);
                                        }
                                    }
                                    // Flush remaining
                                    if !current_chunk.is_empty() {
                                        let chunk = current_chunk.join("\n");
                                        sections.push((in_code_block, chunk));
                                    }

                                    sections.into_iter().flat_map(|(is_code, section)| {
                                        if is_code {
                                            // Handle Code Block
                                            let lines: Vec<&str> = section.lines().collect();
                                            let content = if lines.len() >= 2 {
                                                lines[1..lines.len()-1].join("\n")
                                            } else {
                                                section.replace("```", "")
                                            };
                                            vec![view! { <CodeBlock code=content /> }.into_any()]
                                        } else {
                                            // Handle Text/Concept Blocks (split by \n\n)
                                            section.split("\n\n")
                                                .map(|s| s.trim())
                                                .filter(|s| !s.is_empty())
                                                .map(|s| s.to_string())
                                                .map(|section| {
                                                    enum Segment {
                                                        Plain(String),
                                                        Bold(String),
                                                    }

                                                    let mut segments = Vec::new();

                                                    if section.starts_with("**Concept:") {
                                                        let lines = section.lines().collect::<Vec<_>>();
                                                        let title = lines.first().copied().unwrap_or("")
                                                            .trim_start_matches("**Concept:")
                                                            .trim_end_matches("**")
                                                            .trim()
                                                            .to_string();

                                                        let body_text = if lines.len() > 1 {
                                                            lines[1..].join("\n")
                                                        } else {
                                                            String::new()
                                                        };

                                                        let mut current_pos = 0;
                                                        while let Some(start) = body_text[current_pos..].find("**") {
                                                            let start_idx = current_pos + start;
                                                            if let Some(end) = body_text[start_idx + 2..].find("**") {
                                                                let end_idx = start_idx + 2 + end;
                                                                segments.push(Segment::Plain(body_text[current_pos..start_idx].to_string()));
                                                                segments.push(Segment::Bold(body_text[start_idx + 2..end_idx].to_string()));
                                                                current_pos = end_idx + 2;
                                                            } else { break; }
                                                        }
                                                        segments.push(Segment::Plain(body_text[current_pos..].to_string()));

                                                        view! {
                                                            <ConceptCard title=title>
                                                                {segments.into_iter().map(|s| match s {
                                                                    Segment::Plain(t) => view! { <span>{t}</span> }.into_any(),
                                                                    Segment::Bold(t) => view! { <strong class="text-indigo-400 font-bold">{t}</strong> }.into_any(),
                                                                }).collect_view()}
                                                            </ConceptCard>
                                                        }.into_any()
                                                    } else {
                                                        let mut current_pos = 0;
                                                        while let Some(start) = section[current_pos..].find("**") {
                                                            let start_idx = current_pos + start;
                                                            if let Some(end) = section[start_idx + 2..].find("**") {
                                                                let end_idx = start_idx + 2 + end;
                                                                segments.push(Segment::Plain(section[current_pos..start_idx].to_string()));
                                                                segments.push(Segment::Bold(section[start_idx + 2..end_idx].to_string()));
                                                                current_pos = end_idx + 2;
                                                            } else { break; }
                                                        }
                                                        segments.push(Segment::Plain(section[current_pos..].to_string()));

                                                        view! {
                                                            <p class="text-lg leading-relaxed text-slate-300 whitespace-pre-wrap">
                                                                {segments.into_iter().map(|s| match s {
                                                                    Segment::Plain(t) => view! { <span>{t}</span> }.into_any(),
                                                                    Segment::Bold(t) => view! { <strong class="text-indigo-400 font-bold">{t}</strong> }.into_any(),
                                                                }).collect_view()}
                                                            </p>
                                                        }.into_any()
                                                    }
                                                }).collect::<Vec<_>>()
                                        }
                                    }).collect_view()
                                }
                            </div>

                            {if !examples.is_empty() {
                                view! {
                                    <div class="mt-12 space-y-8">
                                        <h2 class="text-2xl font-semibold text-slate-200 flex items-center gap-2">
                                            <span class="w-8 h-px bg-indigo-500/50"></span>
                                            "コード例"
                                        </h2>
                                        {examples.into_iter().map(|code| view! {
                                            <CodeBlock code=code />
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}

                            {if !learning_points.is_empty() {
                                view! {
                                    <div class="mt-16 p-8 border border-indigo-500/20 bg-indigo-500/5 rounded-2xl relative overflow-hidden group">
                                        <div class="absolute -bottom-2 -right-2 p-4 opacity-10 group-hover:opacity-20 transition-opacity pointer-events-none rotate-12">
                                            <svg xmlns="http://www.w3.org/2000/svg" width="80" height="80" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="text-indigo-400">
                                                <path d="M12 2v8"/><path d="m4.93 10.93 1.41 1.41"/><path d="M2 18h2"/><path d="M20 18h2"/><path d="m19.07 10.93-1.41 1.41"/><path d="M22 22H2"/><path d="m8 22 4-10 4 10"/>
                                            </svg>
                                        </div>

                                        <h3 class="flex items-center gap-2 text-indigo-300 mb-6 font-bold text-xl">
                                            <span class="w-3 h-3 rounded-full bg-indigo-500 shadow-[0_0_10px_rgba(99,102,241,0.5)]"></span>
                                            "学習のポイント"
                                        </h3>

                                        <ul class="space-y-4">
                                            {learning_points.into_iter().map(|point| view! {
                                                <li class="flex gap-4 items-start text-slate-300">
                                                    <div class="mt-1.5 w-1.5 h-1.5 rounded-full bg-indigo-500/50 shrink-0"></div>
                                                    <p>{point}</p>
                                                </li>
                                            }).collect_view()}
                                        </ul>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}

                            <div class="mt-20 flex flex-col md:flex-row justify-between items-center gap-6 pt-8 border-t border-indigo-500/20">
                                {match prev {
                                    Some((id, title)) => view! {
                                        <A href={format!("/lessons/{}", id)} attr:class="group w-full md:w-auto flex flex-col items-start gap-1 p-4 rounded-xl hover:bg-white/5 transition-all text-left">
                                            <span class="text-sm text-slate-400 group-hover:text-indigo-400 transition-colors flex items-center gap-1">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>
                                                "前のレッスン"
                                            </span>
                                            <span class="text-lg font-semibold text-slate-200 group-hover:text-white">{title}</span>
                                        </A>
                                    }.into_any(),
                                    None => view! { <div class="w-full md:w-auto"></div> }.into_any()
                                }}

                                {match next {
                                    Some((id, title)) => view! {
                                        <A href={format!("/lessons/{}", id)} attr:class="group w-full md:w-auto flex flex-col items-end gap-1 p-4 rounded-xl hover:bg-white/5 transition-all text-right">
                                            <span class="text-sm text-slate-400 group-hover:text-indigo-400 transition-colors flex items-center gap-1">
                                                "次のレッスン"
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>
                                            </span>
                                            <span class="text-lg font-semibold text-slate-200 group-hover:text-white">{title}</span>
                                        </A>
                                    }.into_any(),
                                    None => view! { <div class="w-full md:w-auto"></div> }.into_any()
                                }}
                            </div>
                        </div>
                    }.into_any()
                },
                None => view! {
                    <div class="py-20 text-center">
                        <div class="inline-block p-4 rounded-full bg-red-500/10 text-red-400 mb-6">
                            <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold mb-4">"レッスンが見つかりません"</h2>
                        <p class="text-slate-400 mb-8">"お探しのレッスンは削除されたか、URLが正しくない可能性があります。"</p>
                        <A href="/" attr:class="px-6 py-3 bg-indigo-600 hover:bg-indigo-500 rounded-lg font-medium transition-colors">
                            "トップへ戻る"
                        </A>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
