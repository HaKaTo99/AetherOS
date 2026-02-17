//! Ability Marketplace (Phase 27.2)
//! Facilitates P2P resource trading and ability sharing between mesh nodes.

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use crate::enterprise::audit::{AuditSeverity, log_security};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Compute,
    Memory,
    Storage,
    Bandwidth,
}

pub struct TradeOffer {
    pub seller: String,
    pub rtype: ResourceType,
    pub amount: u64,
    pub price_at: u64, // Price in Aether Tokens
}

pub struct MarketplaceManager {
    active_offers: Vec<TradeOffer>,
}

impl MarketplaceManager {
    pub const fn new() -> Self {
        Self { active_offers: Vec::new() }
    }

    pub fn publish_offer(&mut self, node: &str, rtype: ResourceType, amount: u64, price: u64) {
        self.active_offers.push(TradeOffer {
            seller: String::from(node),
            rtype,
            amount,
            price_at: price,
        });
        log_security(AuditSeverity::Info, "Marketplace", &format!("Node {} published offer: {:?} {} units @ {} AT.", node, rtype, amount, price));
    }

    pub fn list_offers(&self) -> &Vec<TradeOffer> {
        &self.active_offers
    }
}

pub static MARKETPLACE: Mutex<MarketplaceManager> = Mutex::new(MarketplaceManager::new());
