use std::{path::PathBuf, process::Command};

use anyhow::Result;

use crate::{config::shell::LocalShellConfig, shell::ShellProvider};

pub struct LocalShell {
    workdir: PathBuf,
}

impl LocalShell {
    pub async fn new(config: LocalShellConfig) -> Result<Self> {
        Ok(Self {
            workdir: PathBuf::from(config.path),
        })
    }
}

#[async_trait::async_trait]
impl ShellProvider for LocalShell {
    async fn exec(&self, commands: String) -> Result<String> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &commands])
                .current_dir(self.workdir.clone())
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .args([&commands])
                .current_dir(self.workdir.clone())
                .output()
        }?;

        Ok(String::from_utf8(output.stdout)?)
    }
}
