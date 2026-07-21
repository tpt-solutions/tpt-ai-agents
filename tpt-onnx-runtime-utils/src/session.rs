use crate::{Error, Tensor};

/// ONNX inference session (mock — real ONNX runtime integration pending).
pub struct Session {
    #[allow(dead_code)]
    model_path: alloc::string::String,
}

impl Session {
    /// Open a session from a model file path.
    pub fn from_file(path: &str) -> Result<Self, Error> {
        if path.is_empty() {
            return Err(Error::ModelLoad(alloc::string::String::from(
                "model path cannot be empty",
            )));
        }
        // In a real implementation, this would validate the file exists
        // and load the ONNX model. Currently a stub.
        Ok(Self {
            model_path: alloc::string::String::from(path),
        })
    }

    /// Run inference (currently a passthrough — real ONNX runtime pending).
    pub fn run(&self, input: &Tensor) -> Result<Tensor, Error> {
        // Stub: returns a zero-filled tensor of the same shape.
        // A real implementation would call the ONNX runtime here.
        let output_data: alloc::vec::Vec<f32> = alloc::vec![0.0; input.len()];
        Tensor::from_slice(&output_data, input.shape())
    }

    pub fn input_names(&self) -> &[&str] {
        &["input"]
    }

    pub fn output_names(&self) -> &[&str] {
        &["output"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_file_empty_path() {
        assert!(Session::from_file("").is_err());
    }

    #[test]
    fn test_from_file_valid_path() {
        assert!(Session::from_file("model.onnx").is_ok());
    }

    #[test]
    fn test_run_passthrough() {
        let session = Session::from_file("model.onnx").unwrap();
        let input = Tensor::from_slice(&[1.0, 2.0, 3.0], &[3]).unwrap();
        let output = session.run(&input).unwrap();
        assert_eq!(output.shape(), &[3]);
        assert_eq!(output.data(), &[0.0, 0.0, 0.0]);
    }
}
