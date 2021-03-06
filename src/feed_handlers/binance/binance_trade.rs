use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Symbol {
    BNBBTC
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinanceTrade {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    event_time: usize,
    #[serde(rename = "s")]
    symbol: Symbol,
    #[serde(rename = "t")]
    trade_id: usize,
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
    #[serde(rename = "b")]
    buyer_order_id: usize,
    #[serde(rename = "a")]
    seller_order_id: usize,
    #[serde(rename = "T")]
    trade_time: usize,
    #[serde(rename = "m")]
    buyer_is_market_maker: bool,
}

impl BinanceTrade {
    pub fn price(&self) -> &str {
        &self.price
    }

    pub fn quantity(&self) -> &str {
        &self.quantity
    }

    pub fn trade_time(&self) -> usize {
        self.trade_time
    }
}