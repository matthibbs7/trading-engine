use std::collections::HashMap;

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask
}

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, price: f64, order: Order) {
        let price = Price::new(price);

        match order.bid_or_ask {
            BidOrAsk::Bid => match self.bids.get_mut(&price) {
                // check if already limit, add order at certain price
                // if not create new price limit
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            }
             
            BidOrAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional
        }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>,
}

// bucket sitting on price level containing bunch of orders from different people of different sizes
impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0
                },
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0
                },
            }

            if market_order.is_filled() {
                break;
            }
        }
    }

    // update orders ledger
    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

// New Order
impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn limit_order_multi_fill() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order_a = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 199.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.orders.get(1).unwrap().size, 1.0);
    }

    #[test]
    fn limit_order_single_fill() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 99.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0);
    }
}