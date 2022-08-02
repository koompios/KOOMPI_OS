// external libraries
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// local libraries
use crate::Architecture;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub architecture: Architecture,
    pub built_time: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Dependencies {
    pub run: Option<HashMap<String, u64>>,
    pub build: Option<HashMap<String, u64>>,
    pub test: Option<HashMap<String, u64>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Build {
    pub script: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Manifest {
    pub package: Package,
    pub dependencies: Option<Dependencies>,
    pub build: Build,
}
