use std::sync::mpsc;

use dotenv::dotenv;

use feed_handlers::BinanceTrade;
use crate::logger::{Logger, FileLogWriter};

use normalizers::Normalizer;

mod feed_handlers;
mod logger;
mod normalizers;

pub struct Config {
    pub server_url: String,
}

fn main() {
    let config: Config = parse_config();

    let (tx, rx) = mpsc::channel::<BinanceTrade>();

    std::thread::spawn(move || feed_handlers::run_binance_feed_handler(&config, tx.clone()));

    let mut logger = Logger::new(FileLogWriter::new("binance_json_feed").unwrap());

    for trade in rx {
        logger.log(&format!("{:?}", trade.normalize())).unwrap();
    }
}

fn parse_config() -> Config {
    dotenv().ok();

    let server_url = dotenv::var("SERVER_URL").expect("Missing 'SERVER_URL' variable");

    Config { server_url }
}
