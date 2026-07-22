#![allow(dead_code)]

use tpt_tool_use_macros::tool;

/// Get the current weather for a location.
#[tool]
fn get_weather(location: String, units: Option<String>) -> String {
    match units {
        Some(u) => format!("Weather for {location} in {u}"),
        None => format!("Weather for {location}"),
    }
}

#[test]
fn test_tool_schema_exists() {
    let schema = get_weather_schema();
    assert!(schema.contains("object"));
    assert!(schema.contains("\"location\""));
    assert!(schema.contains("\"units\""));
}

#[test]
fn test_tool_schema_has_description() {
    let schema = get_weather_schema();
    assert!(schema.contains("Get the current weather"));
}

#[test]
fn test_tool_schema_is_valid_json() {
    let schema = get_weather_schema();
    let value: serde_json::Value = serde_json::from_str(schema).expect("schema must be valid JSON");
    assert_eq!(value["required"], serde_json::json!(["location"]));
}

#[test]
fn test_generated_args_struct_deserializes() {
    let args: GetWeatherArgs =
        serde_json::from_str(r#"{"location":"Seattle","units":"metric"}"#).unwrap();
    assert_eq!(args.location, "Seattle");
    assert_eq!(args.units.as_deref(), Some("metric"));
}

#[test]
fn test_generated_call_function_dispatches() {
    let result = get_weather_call(r#"{"location":"Seattle","units":"metric"}"#).unwrap();
    assert_eq!(result, "Weather for Seattle in metric");
}

#[test]
fn test_generated_call_function_missing_optional() {
    let result = get_weather_call(r#"{"location":"Boston"}"#).unwrap();
    assert_eq!(result, "Weather for Boston");
}

#[test]
fn test_generated_call_function_invalid_json_errors() {
    assert!(get_weather_call(r#"{"not_a_field":true}"#).is_err());
}
