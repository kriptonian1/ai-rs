use super::json_value::{JSONValue};

pub(super) fn is_json_value(value: &JSONValue) -> bool {
    match value {
        JSONValue::Null => true,
        JSONValue::Bool(_) => true,
        JSONValue::Number(_) => true,
        JSONValue::String(_) => true,
        JSONValue::JSONArray(arr) => arr.iter().all(is_json_value),
        JSONValue::JSONObject(obj) => obj.values().all(is_json_value),
        JSONValue::Undefined => true,
    }
}



pub(super) fn is_json_array(value: &JSONValue) -> bool {
    matches!(value, JSONValue::JSONArray(_))
}

pub(super) fn is_json_object(value: &JSONValue) -> bool {
    matches!(value, JSONValue::JSONObject(_))
}