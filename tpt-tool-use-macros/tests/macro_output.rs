#![allow(dead_code, unused_variables, non_upper_case_globals)]

use tpt_tool_use_macros::tool;

/// Get the current weather for a location.
#[tool]
fn get_weather(location: String, _units: Option<String>) -> String {
    format!("Weather for {location}")
}

#[test]
fn test_tool_schema_exists() {
    let schema = get_weather_schema();
    assert!(schema.contains("object"));
    assert!(schema.contains("location"));
}

#[test]
fn test_tool_schema_has_description() {
    let schema = get_weather_schema();
    assert!(schema.contains("Get the current weather"));
}
