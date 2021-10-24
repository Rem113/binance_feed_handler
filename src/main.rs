use dotenv::dotenv;

mod trade;
mod feed_handler;
mod logger;

pub struct Config {
    pub server_url: String
}

fn main() {
    let config: Config = parse_config();

    feed_handler::run(&config);
}

fn parse_config() -> Config {
    dotenv().ok();

    let server_url = dotenv::var("SERVER_URL").expect("Missing 'SERVER_URL' variable");

    Config { server_url }
}
