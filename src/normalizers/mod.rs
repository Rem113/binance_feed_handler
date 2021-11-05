use crate::normalizers::normalized_trade::NormalizedTrade;

mod identity_normalizer;
mod normalized_trade;

pub(crate) trait Normalizer {
    fn normalize(&self) -> NormalizedTrade;
}