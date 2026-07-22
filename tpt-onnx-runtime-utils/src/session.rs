use crate::{Error, Tensor};

#[cfg(feature = "std")]
type RunnablePlan = tract_onnx::prelude::TypedRunnableModel<tract_onnx::prelude::TypedModel>;

/// ONNX inference session.
///
/// With the `std` feature (default), this loads and runs real ONNX models via
/// the pure-Rust [`tract-onnx`](https://docs.rs/tract-onnx) engine. Without
/// `std`, model loading only records the path and `run` returns an error,
/// since `tract-onnx` requires the standard library.
pub struct Session {
    #[allow(dead_code)]
    model_path: alloc::string::String,
    #[cfg(feature = "std")]
    plan: RunnablePlan,
}

impl Session {
    /// Open a session from a model file path.
    pub fn from_file(path: &str) -> Result<Self, Error> {
        if path.is_empty() {
            return Err(Error::ModelLoad(alloc::string::String::from(
                "model path cannot be empty",
            )));
        }

        #[cfg(feature = "std")]
        {
            use tract_onnx::prelude::*;

            let plan = tract_onnx::onnx()
                .model_for_path(path)
                .map_err(|e| Error::ModelLoad(alloc::format!("failed to load model: {e}")))?
                .into_optimized()
                .map_err(|e| Error::ModelLoad(alloc::format!("failed to optimize model: {e}")))?
                .into_runnable()
                .map_err(|e| {
                    Error::ModelLoad(alloc::format!("failed to build runnable plan: {e}"))
                })?;

            Ok(Self {
                model_path: alloc::string::String::from(path),
                plan,
            })
        }

        #[cfg(not(feature = "std"))]
        {
            Ok(Self {
                model_path: alloc::string::String::from(path),
            })
        }
    }

    /// Run inference on the loaded model.
    #[cfg(feature = "std")]
    pub fn run(&self, input: &Tensor) -> Result<Tensor, Error> {
        use alloc::vec;
        use tract_onnx::prelude::*;

        let shape: alloc::vec::Vec<usize> = input.shape().to_vec();
        let data: alloc::vec::Vec<f32> = input.data().to_vec();
        let array = tract_ndarray::ArrayD::from_shape_vec(shape, data)
            .map_err(|e| Error::Inference(alloc::format!("failed to build input array: {e}")))?;
        let input_tensor: tract_onnx::prelude::Tensor = array.into();

        let outputs = self
            .plan
            .run(tvec!(input_tensor.into()))
            .map_err(|e| Error::Inference(alloc::format!("inference failed: {e}")))?;

        let output = outputs.first().ok_or_else(|| {
            Error::Inference(alloc::string::String::from("model produced no outputs"))
        })?;
        let view = output
            .to_array_view::<f32>()
            .map_err(|e| Error::Inference(alloc::format!("failed to read output tensor: {e}")))?;

        let out_shape: alloc::vec::Vec<usize> = view.shape().to_vec();
        let out_data: alloc::vec::Vec<f32> = view.iter().copied().collect();
        crate::Tensor::from_slice(&out_data, &out_shape)
    }

    /// Run inference on the loaded model.
    ///
    /// Unavailable without the `std` feature — `tract-onnx` requires the
    /// standard library, so this always returns [`Error::Inference`].
    #[cfg(not(feature = "std"))]
    pub fn run(&self, _input: &Tensor) -> Result<Tensor, Error> {
        Err(Error::Inference(alloc::string::String::from(
            "ONNX inference requires the 'std' feature (tract-onnx backend)",
        )))
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
    use alloc::string::String;

    #[test]
    fn test_from_file_empty_path() {
        assert!(Session::from_file("").is_err());
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_from_file_nonexistent_path_errors() {
        assert!(Session::from_file("this_model_does_not_exist.onnx").is_err());
    }

    /// Builds a minimal valid ONNX model (a single `Identity` node mapping a
    /// 1-D float tensor of length 3 to itself) using tract-onnx's own
    /// generated protobuf types, so tests exercise real model loading and
    /// real inference rather than a stub.
    #[cfg(feature = "std")]
    fn write_identity_model() -> std::path::PathBuf {
        use prost::Message;
        use tract_onnx::pb::{
            tensor_proto::DataType, tensor_shape_proto::Dimension, type_proto, GraphProto,
            ModelProto, NodeProto, OperatorSetIdProto, TensorShapeProto, TypeProto, ValueInfoProto,
        };

        let tensor_type = |dim: i64| TypeProto {
            denotation: String::new(),
            value: Some(type_proto::Value::TensorType(type_proto::Tensor {
                elem_type: DataType::Float as i32,
                shape: Some(TensorShapeProto {
                    dim: alloc::vec![Dimension {
                        denotation: String::new(),
                        value: Some(
                            tract_onnx::pb::tensor_shape_proto::dimension::Value::DimValue(dim)
                        ),
                    }],
                }),
            })),
        };

        let model = ModelProto {
            ir_version: 8,
            opset_import: alloc::vec![OperatorSetIdProto {
                domain: String::new(),
                version: 13,
            }],
            producer_name: String::from("tpt-onnx-runtime-utils-tests"),
            graph: Some(GraphProto {
                name: String::from("identity_graph"),
                node: alloc::vec![NodeProto {
                    input: alloc::vec![String::from("input")],
                    output: alloc::vec![String::from("output")],
                    name: String::from("identity_node"),
                    op_type: String::from("Identity"),
                    ..Default::default()
                }],
                input: alloc::vec![ValueInfoProto {
                    name: String::from("input"),
                    r#type: Some(tensor_type(3)),
                    doc_string: String::new(),
                }],
                output: alloc::vec![ValueInfoProto {
                    name: String::from("output"),
                    r#type: Some(tensor_type(3)),
                    doc_string: String::new(),
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let bytes = model.encode_to_vec();
        let path = std::env::temp_dir().join(alloc::format!(
            "tpt_onnx_identity_test_{}.onnx",
            std::process::id()
        ));
        std::fs::write(&path, bytes).expect("failed to write test model");
        path
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_real_inference_identity_model() {
        let path = write_identity_model();
        let session = Session::from_file(path.to_str().unwrap()).expect("model should load");
        let input = Tensor::from_slice(&[1.0, 2.0, 3.0], &[3]).unwrap();
        let output = session.run(&input).expect("inference should succeed");
        assert_eq!(output.shape(), &[3]);
        assert_eq!(output.data(), &[1.0, 2.0, 3.0]);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    #[cfg(not(feature = "std"))]
    fn test_run_without_std_errors() {
        // Without the `std` feature, `from_file` still records the path, but
        // `run` cannot invoke tract-onnx and must return an error.
        let session = Session::from_file("model.onnx").unwrap();
        let input = Tensor::from_slice(&[1.0, 2.0, 3.0], &[3]).unwrap();
        assert!(session.run(&input).is_err());
    }
}
