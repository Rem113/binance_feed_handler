use crate::feed_handlers::BinanceTrade;
use crate::feed_handlers::normalized_trade::NormalizedTrade;

impl BinanceTrade {
    pub fn normalize(&self) -> NormalizedTrade {
        let timestamp = self.trade_time();
        let price = String::from(self.price());
        let quantity = String::from(self.quantity());

        NormalizedTrade::new(timestamp, price, quantity)
    }
}