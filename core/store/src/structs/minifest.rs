use super::{BuildScript, Dependency, Package};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Manifest {
    pub package: Package,
    pub dependencies: Option<Dependency>,
    pub build: BuildScript,
}
