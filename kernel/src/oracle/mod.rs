//! Oracle Engine - ML-based Predictive Allocation
//! TinyML predictor for memory management

/// Simple decision tree for memory prediction
pub struct TinyMLPredictor {
    // Historical allocation sizes
    history: [usize; 16],
    history_index: usize,
}

impl TinyMLPredictor {
    pub const fn new() -> Self {
        Self {
            history: [0; 16],
            history_index: 0,
        }
    }

    /// Record an allocation for learning
    pub fn record_allocation(&mut self, size: usize) {
        self.history[self.history_index] = size;
        self.history_index = (self.history_index + 1) % 16;
    }

    /// Predict next allocation size
    pub fn predict_next_size(&self) -> usize {
        // Simple moving average
        let mut sum = 0;
        let mut count = 0;
        
        for &size in &self.history {
            if size > 0 {
                sum += size;
                count += 1;
            }
        }
        
        if count > 0 {
            sum / count
        } else {
            4096 // Default 4KB
        }
    }

    /// Predict if allocation should be distributed
    pub fn should_distribute(&self, size: usize) -> bool {
        // If size > 10MB, recommend distribution
        size > 10 * 1024 * 1024
    }

    /// Predict GC trigger threshold
    pub fn predict_gc_threshold(&self, current_usage: usize, total: usize) -> usize {
        let utilization = (current_usage * 100) / total;
        
        if utilization > 90 {
            // Aggressive GC
            total * 85 / 100
        } else if utilization > 70 {
            // Normal GC
            total * 80 / 100
        } else {
            // Lazy GC
            total * 90 / 100
        }
    }
}

/// Anomaly detection for security
pub struct AnomalyDetector {
    baseline_allocation_rate: usize,
    spike_threshold: usize,
}

impl AnomalyDetector {
    pub const fn new() -> Self {
        Self {
            baseline_allocation_rate: 0,
            spike_threshold: 0,
        }
    }

    /// Detect abnormal allocation patterns
    pub fn detect_anomaly(&self, current_rate: usize) -> bool {
        if self.baseline_allocation_rate == 0 {
            return false;
        }
        
        // If current rate > 5x baseline, it's anomalous
        current_rate > self.baseline_allocation_rate * 5
    }

    /// Update baseline
    pub fn update_baseline(&mut self, rate: usize) {
        // Exponential moving average
        if self.baseline_allocation_rate == 0 {
            self.baseline_allocation_rate = rate;
        } else {
            self.baseline_allocation_rate = 
                (self.baseline_allocation_rate * 9 + rate) / 10;
        }
        
        self.spike_threshold = self.baseline_allocation_rate * 5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prediction() {
        let mut predictor = TinyMLPredictor::new();
        
        // Record some allocations
        predictor.record_allocation(1024);
        predictor.record_allocation(2048);
        predictor.record_allocation(1536);
        
        let predicted = predictor.predict_next_size();
        assert!(predicted > 1000 && predicted < 3000);
    }

    #[test]
    fn test_distribution_decision() {
        let predictor = TinyMLPredictor::new();
        
        assert!(!predictor.should_distribute(1024));
        assert!(predictor.should_distribute(20 * 1024 * 1024));
    }

    #[test]
    fn test_anomaly_detection() {
        let mut detector = AnomalyDetector::new();
        
        detector.update_baseline(1000);
        assert!(!detector.detect_anomaly(2000));
        assert!(detector.detect_anomaly(10000));
    }
}
