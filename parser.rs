use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct OSMElement {
    pub id: u64,
    #[serde(rename = "type")]
    pub element_type: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub nodes: Option<Vec<u64>>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OSMDocument {
    pub elements: Vec<OSMElement>,
}

impl OSMDocument {
    pub fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap_or(OSMDocument { elements: vec![] })
    }
}
