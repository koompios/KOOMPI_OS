use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Dependency {
    pub run: Option<HashMap<String, u64>>,
    pub build: Option<HashMap<String, u64>>,
    pub test: Option<HashMap<String, u64>>,
}
