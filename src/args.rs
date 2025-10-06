use clap::{Parser, Subcommand, command};

#[derive(Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone)]
enum Command {
    Add {
        description: String,
    },
    Update {
        id: u32,
        description: String,
    },
    Delete {
        id: u32,
    },
    MarkInProgress {
        id: u32,
    },
    MarkDone {
        id: u32,
    },
    List {
        #[command(subcommand)]
        list_command: ListCommand,
    },
}

#[derive(Subcommand, Clone)]
enum ListCommand {
    Done,
    Todo,
    InProgress,
}
