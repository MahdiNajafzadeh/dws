use anyhow::{Context, Result};
use std::process::Command;

pub struct StartUp {
    pub command: &'static str,
}

impl StartUp {
    pub fn spawn(&self) -> Result<()> {
        Command::new("sh")
            .arg("-c")
            .arg(self.command)
            .spawn()
            .with_context(|| format!("failed to execute: {}", self.command))?;
        Ok(())
    }
}
