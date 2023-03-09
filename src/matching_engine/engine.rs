use super::orderbook::{Orderbook};
use std::collections::HashMap;

// ==============================
// Example of trading Pair:
//        BTC -> BASE
//        USD -> QUOTE
// ==============================

pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair {
            base,
            quote
        }
    }
} 

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }
}