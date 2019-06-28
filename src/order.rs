use std::collections::BTreeMap;
use std::collections::VecDeque;


pub enum Side {
        Bid,
        Ask,
}

pub type Symbol = String;
pub type OrderId = String;
pub type OrderSeq = i32;
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
        order_seq: OrderSeq,
        price: Price,
        order_qty: Qty,
        side: Side,

        leaves_qty: Qty,
        cum_qty: Qty,

        ord_status: OrdStatus,
}

struct PriceTime {
        price: Price,
        time: OrderSeq,
        ord_seq: OrderSeq,
}

impl Ord for PriceTime {
        fn cmp(&self, other: &Self) -> Ordering {
                (self.price, self.time).cmp(&(other.price, other.name))
        }
}

impl PartialOrd for PriceTime {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}

impl Eq for PriceTime {
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
        
        fn try_match(order: &mut Order, queue_top: &mut Order) -> Option<Match> {
                assert_ne!(order.Side, queue_top.Side);
                
        }
}

pub struct BinaryHeapOrderBook {
        bid_heap: BinaryHeap<PriceTime>,
        ask_heap: BinaryHeap<PriceTime>,
}

impl OrderBook for BinaryHeapOrderBook {
        
        fn handle_order(&mut self, order: &mut Order) -> Option<Match> {
                let m: Option<Match>;
                match order.side {
                        Ask => {
                                m = self.try_match(order, self.bid_orders
                        }

                        Bid => {
                                m = self.try_match(order, self.ask_orders.,
                        }
                }
                
                match order.ord_status {
                        OrdStatus::New | 
                        OrdStatus::PartiallyFilled => self.enqueue(order),
                        OrdStatus::Filled=> (),
                        _ => (),
                }
        }
}

impl BinaryHeapOrderBook {

        fn enqueue(&mut self, order: Order) {
                match order.side {
                        Ask => self.ask_orders.insert(order.price, order),
                        Bid => self.bid_orders.insert(order.price, order),
                }
        }

        fn max(order_tree: &BTreeMap<Price, OrderQueue>) -> &OrderQueue {
                order_tree.
        }
}