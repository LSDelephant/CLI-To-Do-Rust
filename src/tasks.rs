use serde::{Deserialize, Serialize};
use std::fs;

const FILE: &str = "tasks.json";

#[derive(Serialize, Deserialize)]
pub struct Task {
    title: String,
    done: bool,
}

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        if !std::path::Path::new(FILE).exists() {
            fs::write(FILE, "[]").unwrap();
        }
        let data = fs::read_to_string(FILE).unwrap();
        let tasks: Vec<Task> = serde_json::from_str(&data).unwrap();
        TaskManager { tasks }
    }

    pub fn list(&self) {
        if self.tasks.is_empty() {
            println!("✅ Список завдань порожній.");
        } else {
            println!("📋 Список завдань:");
            for (i, task) in self.tasks.iter().enumerate() {
                let status = if task.done { "✔️" } else { "❌" };
                println!("{}. [{}] {}", i + 1, status, task.title);
            }
        }
    }

    pub fn add(&mut self, title: String) {
        self.tasks.push(Task { title, done: false });
        self.save();
        println!("➕ Завдання додано!");
    }

    pub fn done(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index - 1) {
            task.done = true;
            self.save();
            println!("🎉 Завдання виконано!");
        } else {
            println!("⚠️ Немає такого завдання.");
        }
    }

    fn save(&self) {
        let data = serde_json::to_string_pretty(&self.tasks).unwrap();
        fs::write(FILE, data).unwrap();
    }
}
