use crate::config::Config;
use crate::SELECTOR_COMMAND;

use druid::{ExtEventSink, ImageBuf, Target};

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::str::FromStr;

pub type RunResult = anyhow::Result<ImageBuf>;

pub struct Runner {
    pub config: Config,
}

impl Runner {
    /// Run the script, and report back once it is done running
    pub fn run(&self, sink: ExtEventSink) {
        let config_clone = self.config.clone();
        std::thread::spawn(move || {
            let result = match config_clone.command_or_image() {
                Ok(command) => execute_command(command, &config_clone),
                Err(image) => make_image(image),
            };
            let payload = druid::SingleUse::new(result);
            sink.submit_command(SELECTOR_COMMAND, payload, Target::Auto)
                .expect("Expect the submit command to work");
        });
    }
}

fn execute_command(command: &str, config: &Config) -> RunResult {
    tracing::info!("Executing: {}", &command);
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    let result = parse_response(output, &config)?;
    Ok(result)
}

fn parse_response(output: Output, config: &Config) -> RunResult {
    if let Some(content) = std::str::from_utf8(&output.stdout).ok() {
        tracing::debug!("Command Output: {}", content);
    }
    match output.status.success() {
        true => {
            if let Some(image_path) = config.image() {
                make_image(image_path)
            } else {
                make_image(image_path(&output.stdout)?)
            }
        }
        false => {
            if let Ok(n) = String::from_utf8(output.stderr) {
                anyhow::bail!("Run Error: {}", n)
            } else {
                anyhow::bail!("Run Error")
            }
        }
    }
}

fn image_path(from: &[u8]) -> anyhow::Result<PathBuf> {
    let string = std::str::from_utf8(from)?;
    let last_line = string
        .lines()
        .last()
        .ok_or_else(|| anyhow::anyhow!("No output"))?;
    let path = PathBuf::from_str(last_line)?;
    Ok(path)
}

fn make_image(path: impl AsRef<Path>) -> RunResult {
    let png_data = match ImageBuf::from_file(path.as_ref()) {
        Ok(n) => n,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to read image from {}: {:?}",
                path.as_ref().display(),
                e
            ))
        }
    };
    tracing::info!("Loaded image from {}", path.as_ref().display());
    Ok(png_data)
}
