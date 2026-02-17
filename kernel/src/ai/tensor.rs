//! Tensor operations
//! Minimal tensor implementation for AI inference

use alloc::vec;
use alloc::vec::Vec;

/// N-dimensional Tensor
#[derive(Clone)]
pub struct Tensor {
    /// Tensor shape (dimensions)
    shape: Vec<usize>,
    /// Tensor data (flattened)
    data: Vec<f32>,
}

impl Tensor {
    /// Create new tensor with given shape and data
    pub fn new(shape: Vec<usize>, data: Vec<f32>) -> Result<Self, &'static str> {
        let total_size: usize = shape.iter().product();
        if data.len() != total_size {
            return Err("Data length must match shape");
        }
        
        Ok(Self { shape, data })
    }
    
    /// Create tensor filled with zeros
    pub fn zeros(shape: Vec<usize>) -> Self {
        let total_size: usize = shape.iter().product();
        Self {
            shape,
            data: vec![0.0; total_size],
        }
    }
    
    /// Create tensor filled with ones
    pub fn ones(shape: Vec<usize>) -> Self {
        let total_size: usize = shape.iter().product();
        Self {
            shape,
            data: vec![1.0; total_size],
        }
    }
    
    /// Get tensor shape
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }
    
    /// Get tensor data
    pub fn data(&self) -> &[f32] {
        &self.data
    }
    
    /// Get mutable tensor data
    pub fn data_mut(&mut self) -> &mut [f32] {
        &mut self.data
    }
    
    /// Get total number of elements
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if tensor is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Get value at index (flattened)
    pub fn get(&self, index: usize) -> Option<f32> {
        self.data.get(index).copied()
    }
    
    /// Set value at index (flattened)
    pub fn set(&mut self, index: usize, value: f32) -> Result<(), &'static str> {
        if index < self.data.len() {
            self.data[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    
    /// Element-wise addition
    pub fn add(&self, other: &Tensor) -> Result<Tensor, &'static str> {
        if self.shape != other.shape {
            return Err("Shape mismatch");
        }
        
        let data: Vec<f32> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        
        Ok(Self { shape: self.shape.clone(), data })
    }
    
    /// Element-wise multiplication
    pub fn mul(&self, other: &Tensor) -> Result<Tensor, &'static str> {
        if self.shape != other.shape {
            return Err("Shape mismatch");
        }
        
        let data: Vec<f32> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .collect();
        
        Ok(Self { shape: self.shape.clone(), data })
    }
    
    /// Scalar multiplication
    pub fn scale(&self, scalar: f32) -> Tensor {
        let data: Vec<f32> = self.data.iter().map(|x| x * scalar).collect();
        Self { shape: self.shape.clone(), data }
    }
    
    /// Find maximum value
    pub fn max(&self) -> f32 {
        self.data.iter().copied().fold(f32::NEG_INFINITY, f32::max)
    }
    
    /// Find minimum value
    pub fn min(&self) -> f32 {
        self.data.iter().copied().fold(f32::INFINITY, f32::min)
    }
    
    /// Calculate mean
    pub fn mean(&self) -> f32 {
        if self.data.is_empty() {
            return 0.0;
        }
        self.data.iter().sum::<f32>() / self.data.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tensor_creation() {
        let tensor = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        assert_eq!(tensor.shape(), &[2, 3]);
        assert_eq!(tensor.len(), 6);
    }
    
    #[test]
    fn test_tensor_ops() {
        let a = Tensor::ones(vec![2, 2]);
        let b = Tensor::ones(vec![2, 2]);
        
        let c = a.add(&b).unwrap();
        assert_eq!(c.get(0), Some(2.0));
        
        let d = a.scale(3.0);
        assert_eq!(d.get(0), Some(3.0));
    }
}
