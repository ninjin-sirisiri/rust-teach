use std::collections::HashMap;

mod collections;
mod control_flow;
mod enums;
mod error_handling;
mod functions;
mod hello;
mod intro;
mod ownership;
mod setup;
mod structs;
mod variables;

pub struct Lesson {
    pub title: String,
    pub content: String,
    pub code_examples: Vec<String>,
    pub learning_points: Vec<String>,
}

pub fn get_lessons() -> HashMap<String, Lesson> {
    let mut lessons = HashMap::new();

    lessons.insert("intro".to_string(), intro::get_lesson());
    lessons.insert("setup".to_string(), setup::get_lesson());
    lessons.insert("hello".to_string(), hello::get_lesson());
    lessons.insert("variables".to_string(), variables::get_lesson());
    lessons.insert("functions".to_string(), functions::get_lesson());
    lessons.insert("control-flow".to_string(), control_flow::get_lesson());
    lessons.insert("ownership".to_string(), ownership::get_lesson());
    lessons.insert("structs".to_string(), structs::get_lesson());
    lessons.insert("enums".to_string(), enums::get_lesson());
    lessons.insert("collections".to_string(), collections::get_lesson());
    lessons.insert("error-handling".to_string(), error_handling::get_lesson());

    lessons
}

pub fn get_lesson_order() -> Vec<&'static str> {
    vec![
        "intro",
        "setup",
        "hello",
        "variables",
        "functions",
        "control-flow",
        "ownership",
        "structs",
        "enums",
        "collections",
        "error-handling",
    ]
}
