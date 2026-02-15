//! Quantum Computing Simulator (Phase 19.3)
//! Provides primitives for quantum algorithms (Qubits, Gates).

use alloc::vec::Vec;
use spin::Mutex;

/// Complex Number for Quantum State
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

/// A Qubit State
#[derive(Debug, Clone)]
pub struct Qubit {
    pub alpha: Complex, // Amplitude of |0>
    pub beta: Complex,  // Amplitude of |1>
}

impl Qubit {
    pub fn zero() -> Self {
        Self {
            alpha: Complex::new(1.0, 0.0),
            beta: Complex::new(0.0, 0.0),
        }
    }

    /// Apply Hadamard Gate
    pub fn h_gate(&mut self) {
        // H = 1/sqrt(2) * [[1, 1], [1, -1]]
        // Simulation: Just flip state for demo
        let temp = self.alpha;
        self.alpha = Complex::new((self.alpha.re + self.beta.re) * 0.707, 0.0);
        self.beta = Complex::new((temp.re - self.beta.re) * 0.707, 0.0);
    }
}

/// Quantum Processing Unit (QPU) Simulator
pub struct QuantumSim {
    pub qubits: Vec<Qubit>,
}

impl QuantumSim {
    pub const fn new() -> Self {
        Self { qubits: Vec::new() }
    }

    pub fn allocate_qubit(&mut self) -> usize {
        self.qubits.push(Qubit::zero());
        self.qubits.len() - 1
    }

    pub fn run_measure(&self, qubit_idx: usize) -> bool {
        // Measurement collapses state
        if let Some(q) = self.qubits.get(qubit_idx) {
            // Simplified measurement probability
            q.beta.re.abs() > 0.5
        } else {
            false
        }
    }
}

pub static GLOBAL_QPU: Mutex<QuantumSim> = Mutex::new(QuantumSim::new());
