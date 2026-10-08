use serde_derive::{Deserialize, Serialize};
use std::{any::type_name, borrow::Cow};

use crate::steps::{From, To};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transform {
    pub name: Cow<'static, str>,
    pub from: From,
    pub to: To,
}

impl Transform {
    pub fn new<I, O>(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            from: type_name::<I>().to_string(),
            to: type_name::<O>().to_string(),
        }
    }
}
