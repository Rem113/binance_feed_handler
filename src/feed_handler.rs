use websocket::ClientBuilder;
use websocket::message::OwnedMessage::Text;

use crate::Config;
use crate::trade::Trade;

pub fn run(config: &Config) {
    let mut client_builder = ClientBuilder::new(config.server_url.as_str())
        .expect("Invalid URL");

    let mut client = client_builder.connect(None)
        .expect("Failed to connect to client");

    let message_iterator = client.incoming_messages()
        .filter_map(|message| message.ok())
        .filter(|message| message.is_data());

    for message in message_iterator {
        match message {
            Text(text) => {
                let trade: Trade = serde_json::from_str(&text).unwrap();
                println!("{:#?}", trade);
            }
            _ => { panic!("Got a Binary body, should never get here") }
        }
    }
}