use crate::{Error, Tensor};

/// ONNX inference session.
pub struct Session {
    #[allow(dead_code)]
    model_path: alloc::string::String,
}

impl Session {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        Ok(Self {
            model_path: alloc::string::String::from(path),
        })
    }

    pub fn run(&self, input: &Tensor) -> Result<Tensor, Error> {
        let output_data: alloc::vec::Vec<f32> = input.data().to_vec();
        Tensor::from_slice(&output_data, input.shape())
    }

    pub fn input_names(&self) -> &[&str] {
        &["input"]
    }

    pub fn output_names(&self) -> &[&str] {
        &["output"]
    }
}
