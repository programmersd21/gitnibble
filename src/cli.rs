use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "gitnibble",
    version,
    about = "instant, offline-first .gitignore tui"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(long, global = true)]
    pub yes: bool,

    #[arg(long, global = true)]
    pub dry_run: bool,

    #[arg(long, global = true)]
    pub fetch: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Add { templates: Vec<String> },
    Detect,
    Diff { templates: Vec<String> },
    List,
}
