// dande | inuyasha-engine
// lob matching engine. pure throughput.

use std::collections::{BTreeMap, VecDeque};
use std::time::Instant;

#[derive(Debug, Clone)]
struct Order { id: u64, price: u64, qty: u64, is_bid: bool }

struct OrderBook {
    bids: BTreeMap<u64, VecDeque<Order>>, // price -> orders (highest first natively handled by reverse iter)
    asks: BTreeMap<u64, VecDeque<Order>>, // price -> orders
}

impl OrderBook {
    fn new() -> Self { Self { bids: BTreeMap::new(), asks: BTreeMap::new() } }

    fn add_order(&mut self, mut order: Order) {
        if order.is_bid {
            // aggressive match against asks
            let mut prices_to_remove = Vec::new();
            for (&price, queue) in self.asks.iter_mut() {
                if price > order.price || order.qty == 0 { break; }
                while let Some(mut resting) = queue.pop_front() {
                    let fill = resting.qty.min(order.qty);
                    resting.qty -= fill; order.qty -= fill;
                    if resting.qty > 0 { queue.push_front(resting); }
                    if order.qty == 0 { break; }
                }
                if queue.is_empty() { prices_to_remove.push(price); }
            }
            for p in prices_to_remove { self.asks.remove(&p); }
            if order.qty > 0 { self.bids.entry(order.price).or_insert_with(VecDeque::new).push_back(order); }
        } else {
            // passive add for asks (simplified for brevity)
            self.asks.entry(order.price).or_insert_with(VecDeque::new).push_back(order);
        }
    }
}

fn main() {
    let mut book = OrderBook::new();
    let start = Instant::now();
    
    // simulate 1 million orders tearing through the book
    for i in 0..1_000_000 {
        book.add_order(Order { id: i, price: 100 + (i % 10), qty: 10, is_bid: i % 2 == 0 });
    }
    
    println!("processed 1M orders in {:?}", start.elapsed());
}

