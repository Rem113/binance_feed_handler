#[derive(Debug)]
pub struct NormalizedTrade {
    timestamp: usize,
    price: String,
    amount: String,
}

impl NormalizedTrade {
    pub fn new(timestamp: usize, price: String, amount: String) -> NormalizedTrade {
        NormalizedTrade {
            timestamp,
            price,
            amount,
        }
    }
}