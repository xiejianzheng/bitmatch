use std::collections::BTreeMap;

enum Side {
        Bid,
        Ask,
}

type Symbol = String;
type OrderId = String;
type Price = i32
type Qty = i32

struct Order {
        symbol: Symbol,
        order_id: OrderId,
        price: Price,
        order_qty: Qty,
        side: Side,

        leaves_qty: Qty,
        cum_qty: Qty,
}

struct Match {
        symbol: Symbol,
        bid_order_id: OrderId,
        ask_order_id: OrderId,
        match_qty: Qty,
        match_price: Price,
}

trait OrderBook {
        fn add_order(order: Order) -> Option<Match>;
}

struct BTreeOrderBook {
        bid_orders: BTreeMap<Price, Order>,
        ask_orders: BTreeMap<Price, Order>,
}

impl OrderBook for BTreeOrderBook {
        pub add_order(order: Order) -> Option<Match> {
                        
        }
}