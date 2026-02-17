//! Ability Marketplace (Phase 25.2 / 29.2)
//! Enables decentralized P2P resource trading and capability exchange.

use alloc::vec::Vec;
use alloc::string::String;
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Compute,   // TFLOPS / NPU cycles
    Storage,   // Bytes
    Bandwidth, // bps
    Intelligence, // AI Model inference
}

pub struct MarketplaceBid {
    pub bidder_id: u32,
    pub resource: ResourceType,
    pub quantity: u64,
    pub price_at: u64, // Price in AetherTokens
}

pub struct AbilityMarketplace {
    active_bids: Vec<MarketplaceBid>,
    is_active: bool,
}

impl AbilityMarketplace {
    pub const fn new() -> Self {
        Self {
            active_bids: Vec::new(),
            is_active: true,
        }
    }

    pub fn place_bid(&mut self, bid: MarketplaceBid) -> Result<(), &'static str> {
        if !self.is_active { return Err("Marketplace is offline"); }
        self.active_bids.push(bid);
        Ok(())
    }

    pub fn get_available_capacity(&self, rtype: ResourceType) -> u64 {
        self.active_bids.iter()
            .filter(|b| b.resource == rtype)
            .map(|b| b.quantity)
            .sum()
    }
}

pub static GLOBAL_MARKET: Mutex<AbilityMarketplace> = Mutex::new(AbilityMarketplace::new());
