use std::sync::mpsc;

use dotenv::dotenv;

use feed_handlers::BinanceTrade;

use crate::feed_handlers::NormalizedTrade;
use crate::logging::{FileLogWriter, Logger, StdoutLogWriter};

mod feed_handlers;
mod logging;

pub struct Config {
    pub server_url: String,
}

fn main() {
    let config: Config = parse_config();

    let (tx, rx) = mpsc::channel::<BinanceTrade>();

    std::thread::spawn(move || feed_handlers::run_binance_feed_handler(&config, tx.clone()));

    let mut binance_file_logger = Logger::new(FileLogWriter::new("binance_json_feed").unwrap());
    let mut normalized_file_logger = Logger::new(FileLogWriter::new("normalized_json_feed").unwrap());
    let mut console_logger = Logger::new(StdoutLogWriter {});

    for trade in rx {
        let log = format!("{:?}", trade);
        console_logger.log(&log).unwrap();

        handle_binance_trade(trade.clone(), &mut binance_file_logger);
        handle_normalized_trade(trade.normalize(), &mut normalized_file_logger);
    }
}

fn handle_binance_trade(binance_trade: BinanceTrade, logger: &mut Logger<FileLogWriter>) {
    let log = format!("{:?}", binance_trade);

    logger.log(&log).unwrap();
}

fn handle_normalized_trade(normalized_trade: NormalizedTrade, logger: &mut Logger<FileLogWriter>) {
    let log = format!("{:?}", normalized_trade);

    logger.log(&log).unwrap();
}

fn parse_config() -> Config {
    dotenv().ok();

    let server_url = dotenv::var("SERVER_URL").expect("Missing 'SERVER_URL' variable");

    Config { server_url }
}
