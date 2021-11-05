use websocket::client::sync::Client;
use websocket::ClientBuilder;
use websocket::message::OwnedMessage::{self, Text};
use websocket::websocket_base::stream::sync::NetworkStream;

use crate::Config;
use crate::feed_handlers::binance::binance_trade::BinanceTrade;
use std::sync::mpsc::Sender;

pub fn run(config: &Config, tx: Sender<BinanceTrade>) {
    let mut client_builder = ClientBuilder::new(config.server_url.as_str())
        .expect("Invalid URL");

    let mut client = client_builder.connect(None)
        .expect("Failed to connect to client");

    get_feed_from_binance(&mut client)
        .for_each(|trade| tx.send(trade).unwrap());
}

fn get_feed_from_binance<'a>(client: &'a mut Client<Box<dyn NetworkStream + Send>>) -> impl Iterator<Item = BinanceTrade> + 'a {
    client.incoming_messages()
        .filter_map(|message| message.ok())
        .filter_map(|message| extract_text_from_owned_message(message))
        .filter_map(|message| serde_json::from_str::<BinanceTrade>(&message).ok())
}

fn extract_text_from_owned_message(message: OwnedMessage) -> Option<String> {
    match message {
        Text(text) => Some(text),
        _ => None
    }
}