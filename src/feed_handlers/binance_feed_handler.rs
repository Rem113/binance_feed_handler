use websocket::client::sync::Client;
use websocket::ClientBuilder;
use websocket::message::OwnedMessage;
use websocket::websocket_base::stream::sync::NetworkStream;

use crate::Config;
use crate::logger::{FileLogWriter, Logger, LogWriter};
use crate::trade::Trade;
use websocket::message::OwnedMessage::Text;

pub fn run(config: &Config) {
    let mut client_builder = ClientBuilder::new(config.server_url.as_str())
        .expect("Invalid URL");

    let client = client_builder.connect(None)
        .expect("Failed to connect to client");

    let logger = Logger::new(FileLogWriter::new("log1").unwrap());

    handle_feed_from_binance(client, logger);
}

fn handle_feed_from_binance<W: LogWriter>(mut client: Client<Box<dyn NetworkStream + Send>>, mut logger: Logger<W>) {
    let iterator = client.incoming_messages()
        .filter_map(|message| message.ok())
        .filter_map(|message| extract_text_from_owned_message(message))
        .filter_map(|message| serde_json::from_str::<Trade>(&message).ok());

    for message in iterator {
        if let Err(error) = logger.log(&format!("{:#?}", message)) {
            eprintln!("{}", error);
        }
    }
}

fn extract_text_from_owned_message(message: OwnedMessage) -> Option<String> {
    match message {
        Text(text) => Some(text),
        _ => None
    }
}