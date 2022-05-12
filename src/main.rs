use config::Config;
use clap::Parser;
use config::{File, FileFormat, ConfigError};


static DEFAULT_CONFIG_FILE: &'static str = "config.toml";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path to config file. defaults to config.toml
    #[clap(short, long)]
    config_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let config_path = match args.config_file {
        Some(path)  => path,
        None        => DEFAULT_CONFIG_FILE.to_string(),
    };
    let config = get_config(&config_path).expect(&format!("Could not find config file {}", config_path));
    println!("{:#?}", config);
    println!("auto-kick-timer-minutes = {}", config.get::<String>("auto-kick-timer-minutes").unwrap());
}

fn get_config(config_path: &str) -> Result<Config, ConfigError> {
    let builder = Config::builder()
    .set_default("max-queue-size", "-1")?
    .set_default("auto-kick-timer-minutes", "15")?
    .add_source(File::new(config_path, FileFormat::Toml));
    Ok(builder.build()?)
}