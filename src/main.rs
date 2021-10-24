use dotenv::dotenv;
use websocket::ClientBuilder;
use websocket::message::OwnedMessage::Text;

use crate::trade::Trade;

mod trade;

struct Config {
    server_url: String
}

fn main() {
    let config: Config = parse_config();

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

fn parse_config() -> Config {
    dotenv().ok();

    let server_url = dotenv::var("SERVER_URL").expect("Missing 'SERVER_URL' variable");

    Config { server_url }
}
