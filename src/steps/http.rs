use core::fmt;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::Get => f.write_str("GET"),
            HttpMethod::Post => f.write_str("POST"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HttpSource {
    pub url: String,
    pub method: HttpMethod,
    pub body: Option<Value>,
    pub headers: HashMap<String, String>,
    pub fetched_at: DateTime<Utc>,
}
