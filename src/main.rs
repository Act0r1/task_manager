mod args;
use args::CommandArgs;
use args::Commands;
use chrono::{DateTime, Utc};
use clap::Parser;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize)]
pub struct Task {
    title: String,
    status: TaskStatus,
    time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
enum TaskStatus {
    Done,
    NotFinished,
}

#[derive(Default, Serialize, Deserialize)]
pub struct App {
    pub tasks: Vec<Task>,
}

impl App {
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
        self.save();
    }
    pub fn load() -> Self {
        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open("tasks.json")
            .ok();
        let tasks: Vec<Task> = file
            .and_then(|file| serde_json::from_reader(file).ok())
            .unwrap_or_default();
        Self { tasks }
    }
    pub fn delete(&mut self, number: usize) {
        self.tasks.remove(number);
        self.save()
    }

    pub fn save(&self) {
        let file = std::fs::File::create("tasks.json").unwrap();
        serde_json::to_writer(file, &self.tasks).unwrap();
    }
    pub fn finish(&mut self, number: usize) {
        let mut task = &mut self.tasks[number - 1];
        task.status = TaskStatus::Done;
        self.save();
    }
    pub fn edit(&mut self, number: usize, new_name: String) {
        let mut task = &mut self.tasks[number - 1];
        task.title = new_name;
        self.save();
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Done => write!(f, "Done"),
            Self::NotFinished => write!(f, "NotFinished"),
        }
    }
}

impl Task {
    fn new(title: String) -> Self {
        Self {
            title,
            status: TaskStatus::NotFinished,
            time: Utc::now(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name of task - {}, status - {}, created_time - {}",
            self.title,
            self.status,
            self.time.format("%d/%m/%Y %H:%M")
        )
    }
}

fn main() {
    let args = CommandArgs::parse();
    let mut app = App::load();

    match &args.command {
        Commands::Add { new_task } => app.add(Task::new(new_task.to_string())),
        Commands::List => {
            for (ind, item) in app.tasks.iter().enumerate() {
                let index = ind + 1;
                println!("{}. {}", index, item)
            }
        }
        Commands::Delete { number } => {
            app.delete(*number);
            println!("Success deleted")
        }
        Commands::Finish { number } => app.finish(*number),
        Commands::Edit { old_task, new_task } => app.edit(*old_task, new_task.to_string()),
    }
}
