mod http;
pub use http::*;

mod transform;
pub use transform::*;

use core::fmt;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Step {
    Http(HttpSource),
    Key(String),
    Index(usize),
    Str,
    Array,
    Transform(Transform),
    #[default]
    Root,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Step::Root => f.write_str("root"),
            Step::Http(src) => write!(f, "http {} {}", src.method, src.url),
            Step::Key(key) => write!(f, "key {key:?}"),
            Step::Index(idx) => write!(f, "index {idx}"),
            Step::Str => f.write_str("as_str"),
            Step::Array => f.write_str("as_array"),
            Step::Transform(t) => write!(f, "transform {:?}", t.name),
        }
    }
}
