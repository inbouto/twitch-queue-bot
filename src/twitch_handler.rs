
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::message::{ServerMessage};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct TwitchConnection{
    channel: String,
    messages_receiver: UnboundedReceiver<ServerMessage>,
    client: TwitchIRCClient<twitch_irc::transport::tcp::TCPTransport<twitch_irc::transport::tcp::TLS>, twitch_irc::login::StaticLoginCredentials>,
}

impl TwitchConnection {

    pub async fn new(channel: &str, username: &str, token: Option<String>) -> Self {
        // default configuration is to join chat as anonymous.
        let config = ClientConfig::new_simple(StaticLoginCredentials::new(username.to_string(), token));
        let (incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
    
    
        // join a channel
        // This function only returns an error if the passed channel login name is malformed,
        // so in this simple case where the channel name is hardcoded we can ignore the potential
        // error with `unwrap`.
        client.join(channel.to_owned()).unwrap();
    
        client.say(channel.to_string(), "Hello World!".to_string()).await.unwrap();
    
        // keep the tokio executor alive.
        // If you return instead of waiting the background task will exit.
        TwitchConnection{channel: channel.to_string(), messages_receiver: incoming_messages, client: client}
    }
    
    pub async fn read_message(&mut self) -> Option<String> {
        if let Some(message) = self.messages_receiver.recv().await {
            match message {
                ServerMessage::Privmsg(priv_msg) => Some(priv_msg.message_text),
                _ => None,
            }
        }
        else {
            None
        }
    }

    pub async fn send_message(&self, message: String) -> Result<(), String>{
        let res = self.client.say(self.channel.clone(), message).await;
        if res.is_err(){
            return Err(format!("Could not send message to {} : {}", &self.channel, res.unwrap_err() ))
        }
        Ok(())
    }
}
