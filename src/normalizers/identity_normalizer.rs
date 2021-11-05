use crate::feed_handlers::BinanceTrade;
use crate::normalizers::normalized_trade::NormalizedTrade;
use crate::normalizers::Normalizer;

impl Normalizer for BinanceTrade {
    fn normalize(&self) -> NormalizedTrade {
        NormalizedTrade::new(
            self.trade_time(),
            String::from(self.price()),
            String::from(self.quantity()))
    }
}