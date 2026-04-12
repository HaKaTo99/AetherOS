//! Brain-Computer Interface (BCI) Driver (Phase 19.4)
//! Simulates Neural Link signals for intent-controlled UI.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrainSignal {
    pub alpha: f32, // Relaxation
    pub beta: f32,  // Active focus
    pub gamma: f32, // Peak cognitive load / Intent
}

pub struct NeuralLink {
    connected: bool,
    ticks: u64,
}

impl NeuralLink {
    pub const fn new() -> Self {
        Self { connected: false, ticks: 0 }
    }

    pub fn init(&mut self) {
        self.connected = true;
        crate::println!("[BCI] Neural Link established. Synchronizing with Sovereign Operator.");
    }

    pub fn read_signal(&mut self) -> Option<BrainSignal> {
        if !self.connected { return None; }
        self.ticks += 1;
        
        // Simpler wave-like behavior without sin/cos (avoiding no_std f32 issues)
        let phase = (self.ticks % 100) as f32 / 100.0;
        let alpha = 0.5 + phase * 0.1;
        let beta = 0.7 - phase * 0.05;
        let gamma = if self.ticks % 500 > 480 { 0.95 } else { 0.1 };

        Some(BrainSignal { alpha, beta, gamma })
    }
}

pub static mut GLOBAL_NEURAL_LINK: NeuralLink = NeuralLink::new();
