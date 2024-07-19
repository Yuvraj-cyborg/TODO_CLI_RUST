use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "minor_project")]
#[command(about = "A versatile CLI application for task management")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Show {
        #[arg(short, long)]
        all: bool,
        #[arg(short, long)]
        completed: bool,
        #[arg(short, long)]
        incomplete: bool,
        list_name: Option<String>,
    },
    Add {
        list_name: String,
        item: String,
    },
   
}
