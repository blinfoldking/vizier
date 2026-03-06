use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use clap::Args;
use dirs::home_dir;
use duration_string::DurationString;
use inquire::{Confirm, CustomType, Password, Select, Text};

#[derive(Debug, Args, Clone)]
pub struct OnboardArgs {
    #[arg(short, long, value_name = "PATH", help = "path to workspace")]
    pub path: Option<String>,
}

pub async fn onboard(args: OnboardArgs) -> Result<()> {
    Ok(())
}
