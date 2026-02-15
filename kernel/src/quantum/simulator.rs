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
        // Hadamard: |0> -> (|0> + |1>)/sqrt(2), |1> -> (|0> - |1>)/sqrt(2)
        let inv_sqrt2 = 0.70710678;
        let new_alpha = self.alpha.add(self.beta).scale(inv_sqrt2);
        let new_beta = self.alpha.sub(self.beta).scale(inv_sqrt2);
        self.alpha = new_alpha;
        self.beta = new_beta;
    }

    pub fn x_gate(&mut self) {
        // Pauli-X (NOT): |0> -> |1>, |1> -> |0>
        let temp = self.alpha;
        self.alpha = self.beta;
        self.beta = temp;
    }

    pub fn z_gate(&mut self) {
        // Pauli-Z: |0> -> |0>, |1> -> -|1>
        self.beta = self.beta.scale(-1.0);
    }

    pub fn measure(&mut self) -> bool {
        // Simplified measurement probability
        // A proper measurement would use a random number generator
        // and collapse the state based on probabilities |alpha|^2 and |beta|^2.
        // For this simulation, we'll use a threshold on beta's probability.
        let prob_one = self.beta.norm_sq();
        if prob_one > 0.5 {
            // Collapse to |1>
            self.alpha = Complex::new(0.0, 0.0);
            self.beta = Complex::new(1.0, 0.0);
            true
        } else {
            // Collapse to |0>
            self.alpha = Complex::new(1.0, 0.0);
            self.beta = Complex::new(0.0, 0.0);
            false
        }
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
        self.qubits.push(Qubit::new());
        self.qubits.len() - 1
    }

    pub fn cnot(&mut self, control_idx: usize, target_idx: usize) {
        // Simplified CNOT simulation for separate qubits (not fully entangled state vector)
        // If Control is heavily weighted to |1>, flip Target.
        // Note: Real quantum computing requires a combined state vector (Size 2^N).
        // This is a "Internet of Abilities" simulation.
        if let Some(control) = self.qubits.get(control_idx) {
            if control.beta.norm_sq() > 0.5 {
                if let Some(target) = self.qubits.get_mut(target_idx) {
                    target.x_gate();
                }
            }
        }
    }

    pub fn run_measure(&mut self, idx: usize) -> bool {
        if let Some(qubit) = self.qubits.get_mut(idx) {
            qubit.measure()
        } else {
            false
        }
    }
}

pub static GLOBAL_QPU: Mutex<QuantumSim> = Mutex::new(QuantumSim::new());
