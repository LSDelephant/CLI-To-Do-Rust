mod tasks;
use tasks::TaskManager;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut manager = TaskManager::new();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "list" => manager.list(),
        "add" => {
            if args.len() > 2 {
                let title = args[2..].join(" ");
                manager.add(title);
            } else {
                println!("⚠️ Вкажи назву завдання.");
            }
        }
        "done" => {
            if args.len() > 2 {
                if let Ok(index) = args[2].parse::<usize>() {
                    manager.done(index);
                } else {
                    println!("⚠️ Номер завдання має бути числом.");
                }
            } else {
                println!("⚠️ Вкажи номер завдання.");
            }
        }
        _ => print_help(),
    }
}

fn print_help() {
    println!("Використання:");
    println!(" cargo run -- list              # показати всі завдання");
    println!(" cargo run -- add \"Купити хліб\" # додати завдання");
    println!(" cargo run -- done 1            # відмітити як виконане");
}
