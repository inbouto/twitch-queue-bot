mod commands;
mod twitch_handler;
mod discord_handler;

use config::Config;
use clap::Parser;
use config::{File, FileFormat, ConfigError};
use twitch_handler::{TwitchConnection};
use commands::Command;

static DEFAULT_CONFIG_FILE: &'static str = "config.toml";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path to config file. defaults to config.toml
    #[clap(short, long)]
    config_file: Option<String>,
}



#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config_path = match args.config_file {
        Some(path)  => path,
        None        => DEFAULT_CONFIG_FILE.to_string(),
    };
    let config = get_config(&config_path).expect(&format!("Could not find config file {}", config_path));

    let commands = init_commands();

    let twitch_connection = TwitchConnection::new(&config.get::<String>("twitch-channel-listen").unwrap(), &config.get::<String>("twitch-bot-username").unwrap(), Some(config.get::<String>("twitch-bot-token").unwrap())).await;
    twitch_messages_handler(twitch_connection, &commands, &config.get::<String>("twitch-command-prefix").unwrap()).await;
}

fn get_config(config_path: &str) -> Result<Config, ConfigError> {
    let builder = Config::builder()
    .set_default("max-queue-size", "-1")?
    .set_default("auto-kick-timer-minutes", "15")?
    .add_source(File::new(config_path, FileFormat::Ini));
    Ok(builder.build()?)
}

fn init_commands() -> Vec<Command> {
    let mut commands = Vec::<Command>::new();
    commands.push(Command::new(&"ping", vec![], Some(String::from("Ping, and I'll Pong!")), commands::ping));
    commands
}

async fn twitch_messages_handler(mut twitch_connection: TwitchConnection , commands: &Vec<Command>, command_prefix: &str) {
    loop {
        match twitch_connection.read_message().await {
            Some(message) => {
                let mut words = (&message[..]).split_whitespace();
                let first_word = words.next().unwrap();
                if !(&first_word[..command_prefix.len()] == command_prefix){
                    return;
                }
                match commands.iter().find( | &command | command.clone().get_keyword() == first_word[command_prefix.len()..] ) {
                    Some(command) => twitch_connection.send_message(first_word[command_prefix.len()..].to_string()).await.unwrap_or(()),
                    _ => (),
                };
            },
            _   => (),
        };
    }
}