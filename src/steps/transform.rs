use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transform {
    pub name: Cow<'static, str>,
}

impl Transform {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self { name: name.into() }
    }
}
