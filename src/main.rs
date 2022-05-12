use config::Config;
use clap::Parser;
use config::{File, FileFormat, ConfigError};

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient, irc};
use twitch_irc::message::{IRCMessage, ServerMessage};

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
    println!("{:#?}", config);
    println!("auto-kick-timer-minutes = {}", config.get::<String>("auto-kick-timer-minutes").unwrap());
    twitch_connect(&config.get::<String>("twitch-channel-listen").unwrap(), &config.get::<String>("twitch-bot-username").unwrap(), Some(config.get::<String>("twitch-bot-token").unwrap())).await;
}

fn get_config(config_path: &str) -> Result<Config, ConfigError> {
    let builder = Config::builder()
    .set_default("max-queue-size", "-1")?
    .set_default("auto-kick-timer-minutes", "15")?
    .add_source(File::new(config_path, FileFormat::Ini));
    Ok(builder.build()?)
}




async fn twitch_connect(channel: &str, username: &str, token: Option<String>) {
    // default configuration is to join chat as anonymous.
    let config = ClientConfig::new_simple(StaticLoginCredentials::new(username.to_string(), token));
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(priv_msg) => println!("@{} => {} : {} ", priv_msg.channel_login, priv_msg.sender.name, priv_msg.message_text),
                _ => (),
            }
        }
    });

    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    // so in this simple case where the channel name is hardcoded we can ignore the potential
    // error with `unwrap`.
    client.join(channel.to_owned()).unwrap();

    client.say(channel.to_string(), "Hello World!".to_string()).await.unwrap();

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}

