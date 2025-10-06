use clap::{Parser, Subcommand};

#[derive(Parser)]
pub(crate) struct CliArgs {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Clone)]
pub(crate) enum Command {
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
pub(crate) enum ListCommand {
    Done,
    Todo,
    InProgress,
}
