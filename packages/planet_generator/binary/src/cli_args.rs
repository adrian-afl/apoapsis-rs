use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLIArgs {
    #[arg(default_value = "../../../media/universe/solar_system/earth/earth.json")]
    pub input: String,
}
