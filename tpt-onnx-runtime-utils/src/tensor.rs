use crate::Error;

/// A tensor for ONNX inference.
pub struct Tensor {
    data: alloc::vec::Vec<f32>,
    shape: alloc::vec::Vec<usize>,
}

impl Tensor {
    pub fn from_slice(data: &[f32], shape: &[usize]) -> Result<Self, Error> {
        let expected: usize = shape.iter().product();
        if data.len() != expected {
            return Err(Error::InvalidShape {
                expected: alloc::vec![expected],
                actual: alloc::vec![data.len()],
            });
        }
        Ok(Self {
            data: alloc::vec::Vec::from(data),
            shape: alloc::vec::Vec::from(shape),
        })
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_slice_valid() {
        let t = Tensor::from_slice(&[1.0, 2.0, 3.0], &[1, 3]).unwrap();
        assert_eq!(t.data(), &[1.0, 2.0, 3.0]);
        assert_eq!(t.shape(), &[1, 3]);
        assert_eq!(t.len(), 3);
    }

    #[test]
    fn test_from_slice_shape_mismatch() {
        let result = Tensor::from_slice(&[1.0, 2.0], &[1, 3]);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_tensor() {
        let t = Tensor::from_slice(&[], &[0]).unwrap();
        assert!(t.is_empty());
    }
}
