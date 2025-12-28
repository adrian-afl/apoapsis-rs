use clap::Parser;
use std::sync::LazyLock;
use tracing::Level;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct GlobalConfig {
    #[arg(short, long, default_value_t = Level::TRACE)]
    pub log_level: Level,

    #[arg(short, long, default_value_t = false)]
    pub headless: bool,

    #[arg(short, long, default_value = "nats://localhost:4222")]
    pub nats_address: String,
}

pub static GLOBAL_CONFIG: LazyLock<GlobalConfig> = LazyLock::new(GlobalConfig::parse);
