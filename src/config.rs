use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum Mode {
    /// Path to an image and a command that is to be run if any of the output folders change
    CommandImage {
        command: String,
        watch: Vec<PathBuf>,
        image: PathBuf,
    },
    /// Path to an image that is be reloaded if the image changes
    Image { path: PathBuf },
    /// Path to a command that is run when a folder changes and prints (as the last line) the image to load
    CommandOutput {
        command: String,
        watch: Vec<PathBuf>,
    },
}

#[derive(Debug, Clone)]
pub struct Config {
    mode: Mode,
}

impl Config {
    pub fn new(mode: Mode) -> Self {
        Self { mode }
    }

    pub fn watch_paths(&self) -> Vec<PathBuf> {
        match &self.mode {
            Mode::CommandImage { watch, .. } => watch.clone(),
            Mode::Image { path } => vec![path.clone()],
            Mode::CommandOutput { watch, .. } => watch.clone(),
        }
    }

    pub fn image(&self) -> Option<&Path> {
        match &self.mode {
            Mode::CommandImage { image, .. } => Some(&image),
            Mode::Image { path } => Some(&path),
            Mode::CommandOutput { .. } => None,
        }
    }

    /// We're mis-using `Result` as a cheap `Either` type
    pub fn command_or_image(&self) -> std::result::Result<&str, &Path> {
        match &self.mode {
            Mode::CommandImage { command, .. } => Ok(&command),
            Mode::Image { path } => Err(&path),
            Mode::CommandOutput { command, .. } => Ok(&command),
        }
    }
}
