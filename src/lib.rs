use std::{collections::HashMap, ops::Index, panic::Location, sync::Arc};

use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct BuildInfo {
    pub git_sha: &'static str,
    pub dirty: bool,
    pub src_hash: &'static str,
    pub crate_version: &'static str,
}

pub const BUILD: BuildInfo = BuildInfo {
    git_sha: env!("GIT_SHA"),
    dirty: matches!(env!("GIT_DIRTY").as_bytes(), b"true"),
    src_hash: env!("SRC_HASH"),
    crate_version: env!("CARGO_PKG_VERSION"),
};

#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Debug, Clone)]
pub struct HttpSource {
    url: String,
    method: HttpMethod,
    body: Option<Value>,
    headers: HashMap<String, String>,
    fetched_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub enum Source {
    Http(HttpSource),
    Slice(String),
    #[default]
    Root,
}

pub struct SourceTracker<T: Clone> {
    build: BuildInfo,
    trail: Trail,
    data: Option<T>,
}

impl<T: Clone> Default for SourceTracker<T> {
    #[track_caller]
    fn default() -> Self {
        Self {
            build: BUILD,
            trail: Default::default(),
            data: Default::default(),
        }
    }
}

impl<T: Clone> SourceTracker<T> {
    #[track_caller]
    fn add_source(&mut self, step: Source) {
        self.trail = Trail {
            parent: Some(Arc::new(self.trail.clone())),
            step,
            at: Location::caller(),
        }
    }

    #[track_caller]
    fn var<'d>(&'d mut self, data: T) -> TrackedData<'d, T> {
        self.data = Some(data);
        TrackedData {
            trail: self.trail.clone(),
            data: &self.data.as_ref().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Trail {
    parent: Option<Arc<Trail>>,
    step: Source,
    at: &'static Location<'static>,
}

impl Default for Trail {
    #[track_caller]
    fn default() -> Self {
        Self {
            parent: Default::default(),
            step: Default::default(),
            at: Location::caller(),
        }
    }
}

#[derive(Debug)]
pub struct TrackedData<'d, T: Clone> {
    trail: Trail,
    data: &'d T,
}

impl<'d, T: Clone> TrackedData<'d, T> {
    #[track_caller]
    pub fn get(&self, idx: &str) -> TrackedData<'d, T>
    where
        for<'a> T: Index<&'a str, Output = T>,
    {
        let trail = Trail {
            parent: Some(Arc::new(self.trail.clone())),
            step: Source::Slice(idx.to_string()),
            at: Location::caller(),
        };

        TrackedData {
            trail,
            data: &self.data[idx],
        }
    }
}

// First source -> Meta -> second source

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_track_source() {
        let mut source_tracker = SourceTracker::default();

        let body = json! {{
          "NRBR": [
            "NR"
          ],
          "GP_CODE": ["XXVII"],
          "VHG": [
            "BNR"
          ],
          "VHG2": [
            "BNR"
          ],
          "DOKTYP": [
            "BNR"
          ]
        }};

        source_tracker.add_source(Source::Http(HttpSource {
            url: "".to_string(),
            method: HttpMethod::Post,
            body: Some(body),
            headers: HashMap::default(),
            fetched_at: Utc::now(),
        }));

        let var = source_tracker.var(json! {{
            "hi": {
                "idx": "data"
            }
        }});
        let hi = var.get("hi").get("idx");
        dbg!(&hi);
        // let res = hi.get("hi");

        source_tracker.add_source(Source::Http(HttpSource {
            url: "".to_string(),
            method: HttpMethod::Get,
            body: None,
            headers: HashMap::default(),
            fetched_at: Utc::now(),
        }));
    }
}
