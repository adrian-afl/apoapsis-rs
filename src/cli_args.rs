use clap::{Parser, Subcommand};
use tracing::Level;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLIArgs {
    #[arg(short, long, default_value_t = Level::TRACE)]
    pub log_level: Level,

    #[command(subcommand)]
    pub entry: Option<EntrypointOverride>,
}

#[derive(Subcommand)]
pub enum EntrypointOverride {
    BodyViewer {
        #[arg(short, long)]
        name: String,
    },
    OnGroundSandbox,
    InOrbitSandbox,
    LoadSave {
        #[arg(short, long)]
        path: String,
    },
}
