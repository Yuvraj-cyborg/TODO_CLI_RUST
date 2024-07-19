mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use serde_json;

type TaskList = HashMap<String, Vec<String>>;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Show { all, completed, incomplete, list_name } => {
            show_tasks(*all, *completed, *incomplete, list_name.clone());
        }
        Commands::Add { list_name, item } => {
            add_task(list_name, item);
        }
    }
}
fn show_tasks(all: bool, completed: bool, incomplete: bool, list_name: Option<String>) {
    let tasks = load_tasks().unwrap_or_else(|_| {
        println!("No tasks found, initializing empty task list.");
        HashMap::new()
    });

    if all {
        println!("Showing all tasks...");
        for (list, items) in &tasks {
            println!("List: {}", list);
            for item in items {
                println!("  - {}", item);
            }
        }
    } else if completed {
        println!("Showing completed tasks...");
        // Add logic to filter and display completed tasks
    } else if incomplete {
        println!("Showing incomplete tasks...");
        // Add logic to filter and display incomplete tasks
    } else if let Some(name) = list_name {
        println!("Showing tasks for list: {}", name);
        if let Some(items) = tasks.get(&name) {
            for item in items {
                println!("  - {}", item);
            }
        } else {
            println!("No tasks found for list: {}", name);
        }
    } else {
        // Default case: Show all tasks if no specific filter is applied
        println!("Showing all tasks...");
        for (list, items) in &tasks {
            println!("List: {}", list);
            for item in items {
                println!("  - {}", item);
            }
        }
    }
}



fn add_task(list_name: &str, item: &str) {
    let mut tasks = load_tasks().unwrap_or_else(|_| {
        println!("No tasks found, initializing empty task list.");
        HashMap::new()
    });

    // Add item
    tasks.entry(list_name.to_string()).or_insert_with(Vec::new).push(item.to_string());

    // Save tasks
    if let Err(e) = save_tasks(&tasks) {
        println!("Failed to save tasks: {}", e);
    } else {
        println!("Added task '{}' to list '{}'", item, list_name);
    }
}

fn load_tasks() -> io::Result<TaskList> {
    let path = "tasks.json";
    if Path::new(path).exists() {
        let data = fs::read_to_string(path)?;
        let tasks: TaskList = serde_json::from_str(&data)?;
        println!("Loaded tasks: {:?}", tasks); // Debug print
        Ok(tasks)
    } else {
        println!("Task file not found, initializing empty task list.");
        Ok(HashMap::new())
    }
}

fn save_tasks(tasks: &TaskList) -> io::Result<()> {
    let data = serde_json::to_string(tasks)?;
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("tasks.json")?;
    file.write_all(data.as_bytes())?;
    println!("Tasks saved successfully."); // Debug print
    Ok(())
}
