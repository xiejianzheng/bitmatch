use std::collections::BTreeMap;
use std::collections::VecDeque;

pub enum Side {
        Bid,
        Ask,
}

pub type Symbol = String;
pub type OrderId = String;
pub type Price = i32;
pub type Qty = i32;
pub type OrderQueue = VecDeque<Order>;

pub enum OrdStatus {
        New,
        PartiallyFilled,
        Filled
}

pub struct Order {
        symbol: Symbol,
        order_id: OrderId,
        price: Price,
        order_qty: Qty,
        side: Side,

        leaves_qty: Qty,
        cum_qty: Qty,

        ord_status: OrdStatus,
}


pub struct Match {
        symbol: Symbol,
        bid_order_id: OrderId,
        ask_order_id: OrderId,
        match_qty: Qty,
        match_price: Price,
}

pub trait OrderBook {
        fn handle_order(&mut self, order: &mut Order) -> Option<Match>;
}

pub struct BTreeOrderBook {
        bid_orders: BTreeMap<Price, OrderQueue>,
        ask_orders: BTreeMap<Price, OrderQueue>,
}

impl OrderBook for BTreeOrderBook {
        
        fn handle_order(&mut self, order: &mut Order) -> Option<Match> {
                let m: Option<Match>;
                match order.side {
                        Ask => 
                               m = self.try_match(order, self.bid_orders),
                        Bid => m = self.try_match(order, self.ask_orders),
                }
                
                match order.ord_status {
                        OrdStatus::New | 
                        OrdStatus::PartiallyFilled => self.enqueue(order),
                        OrdStatus::Filled=> (),
                        _ => (),
                }
        }
}

impl BTreeOrderBook {
        fn try_match(&mut self, mut order: &Order, otherSide: OrderQueue) -> Option<Match> {
                order.ord_status = OrdStatus::New;
                return Option::None;
        }

        fn enqueue(&mut self, order: Order) {
                match order.side {
                        Ask => self.ask_orders.insert(order.price, order),
                        Bid => self.bid_orders.insert(order.price, order),
                }
        }
}