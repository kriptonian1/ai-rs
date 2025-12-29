use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum JSONValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    JSONArray(Vec<JSONValue>),
    JSONObject(HashMap<String, JSONValue>),
    Undefined,
}

pub(super) type JSONArray = Vec<JSONValue>;

pub(super) type JSONObject = HashMap<String, JSONValue>;