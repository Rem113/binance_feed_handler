use websocket::client::sync::Client;
use websocket::ClientBuilder;
use websocket::message::OwnedMessage::Text;
use websocket::websocket_base::stream::sync::NetworkStream;

use crate::Config;
use crate::logger::{Logger, LogWriter, StdOutLogWriter};
use crate::trade::Trade;

pub fn run(config: &Config) {
    let mut client_builder = ClientBuilder::new(config.server_url.as_str())
        .expect("Invalid URL");

    let client = client_builder.connect(None)
        .expect("Failed to connect to client");

    let logger = Logger::new(StdOutLogWriter);

    listen(client, logger);
}

fn listen<W: LogWriter>(mut client: Client<Box<dyn NetworkStream + Send>>, logger: Logger<W>) {
    let message_iterator = client.incoming_messages()
        .filter_map(|message| message.ok())
        .filter(|message| message.is_data());

    for message in message_iterator {
        match message {
            Text(text) => {
                let trade: Trade = serde_json::from_str(&text).unwrap();
                logger.info(format!("{:#?}", trade)).ok();
            }
            _ => { panic!("Got a Binary body, should never get here") }
        }
    }
}