//! CapTrade Manager (Phase 29.2)
//! Decentralized Resource Trading Engine for Compute/Storage.

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

#[derive(Debug, Clone)]
pub enum ResourceType {
    Compute(u64), // TFLOPS
    Storage(u64), // MB
}

#[derive(Debug, Clone)]
pub struct MarketOrder {
    pub id: u64,
    pub node_id: u64,
    pub resource: ResourceType,
    pub price: u64, // AetherCoins (AT)
}

/// The CapTrade (Ability Economy) Engine
pub struct CapTradeManager {
    buy_orders: Vec<MarketOrder>,
    sell_orders: Vec<MarketOrder>,
}

impl CapTradeManager {
    pub const fn new() -> Self {
        Self {
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        crate::println!("[Market] Capability Exchange Online. Listening for bids/asks...");
    }

    /// Place a Buy Order (Bid)
    pub fn place_bid(&mut self, node_id: u64, resource: ResourceType, price: u64) {
        crate::println!("[Market] Node {} bids {:?} for {} AT", node_id, resource, price);
        self.buy_orders.push(MarketOrder { id: 0, node_id, resource: resource, price });
        self.match_orders();
    }

    /// Place a Sell Order (Ask)
    pub fn place_ask(&mut self, node_id: u64, resource: ResourceType, price: u64) {
        crate::println!("[Market] Node {} asks {:?} for {} AT", node_id, resource, price);
        self.sell_orders.push(MarketOrder { id: 0, node_id, resource: resource, price });
        self.match_orders();
    }

    /// Match orders (Simple FIFO matching)
    fn match_orders(&mut self) {
        // In a real implementation, this would be a complex matching engine
        if !self.buy_orders.is_empty() && !self.sell_orders.is_empty() {
             let bid = self.buy_orders.remove(0);
             let ask = self.sell_orders.remove(0);
             
             crate::println!("[CapTrade] MATCH! Node {} sold to Node {} @ {} AT", ask.node_id, bid.node_id, bid.price);
             // Transaction would be recorded on distributed ledger
        }
    }
}

pub static CAPTRADE_MANAGER: Mutex<CapTradeManager> = Mutex::new(CapTradeManager::new());
