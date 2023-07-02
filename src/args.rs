use clap::{Parser, Subcommand};


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CommandArgs {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List all tasks
    List,
    /// Add new task, remeber, task should be in double quotes
    Add { new_task: String },
    /// Delete task, pass the number of task
    Delete { number: usize },
    /// Edit task, first of all pass number of task that you want change
    /// second, pass the new name in double quotes
    Edit { old_task: usize, new_task: String },
    /// Finish task by number
    Finish { number: usize },
}

