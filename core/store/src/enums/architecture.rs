use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Architecture {
  X86_64,
  AArch64
}

impl Default for Architecture {
    fn default() -> Self {
        Self::X86_64
    }
}