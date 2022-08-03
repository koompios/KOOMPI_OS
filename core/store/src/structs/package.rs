// external libraries
use crate::Architecture;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub architecture: Architecture,
    pub built_time: u64,
}
