#[derive(Debug)]
pub struct NormalizedTrade {
    timestamp: usize,
    price: String,
    quantity: String,
}

impl NormalizedTrade {
    pub fn new(timestamp: usize, price: String, quantity: String) -> NormalizedTrade {
        NormalizedTrade {
            timestamp,
            price,
            quantity,
        }
    }
}