use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BuildScript {
    pub script: String,
}

impl BuildScript {
    pub fn build(&self) {
        let mut cmd = Command::new("sh")
            .arg("-e")
            .arg("-c")
            .arg(&self.script)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        cmd.wait().unwrap();
    }
}
