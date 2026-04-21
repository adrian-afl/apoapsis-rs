use clap::Parser;
use std::sync::LazyLock;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GlobalConfig {
    #[arg(short, long, default_value_t = 7878)]
    pub port: u16,
}

pub static GLOBAL_CONFIG: LazyLock<GlobalConfig> = LazyLock::new(GlobalConfig::parse);
