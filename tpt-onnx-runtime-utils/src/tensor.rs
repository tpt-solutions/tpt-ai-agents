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
