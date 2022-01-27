mod config;
mod delegate;
mod interface;
mod runner;
mod watcher;

use core::panic;
use druid::{Data, ImageBuf, Lens, Selector, SingleUse};
use std::path::PathBuf;

use config::{Config, Mode};

pub const SELECTOR_EVENT: Selector<Vec<PathBuf>> = Selector::new("Event");
pub const SELECTOR_COMMAND: Selector<SingleUse<runner::RunResult>> = Selector::new("Command");

#[derive(Clone, Data, Lens)]
struct WatchoutState {
    image: ImageBuf,
    loading: bool,
    failure: bool,
}

use clap::{app_from_crate, arg, App, Arg};

fn main() {
    setup_tracing();

    let watch_arg = Arg::new("watch")
        .short('w')
        .long("watch")
        .takes_value(true)
        .required(true)
        .help("The folder to watch for changes")
        .multiple_occurrences(true);

    let matches = app_from_crate!()
    .subcommand(
        App::new("command-image")
            .about("Perform [cmd] when the [watch] folder (recursively) changes and then reload [img]")
            .arg(arg!(-c --cmd [cmd] "The command to execute")
            .required(true))
            // .arg(arg!(-w --watch [watch] "The folder to watch for changes").required(true).multiple_occurrences(true))
            .arg(watch_arg.clone())
            .arg(arg!(-i --img [img] "The image to reload when there is a change").required(true))
        )
    .subcommand(
        App::new("image")
            .about("Reload [img] when it changes")
            .arg(arg!(-i --img [img] "The image to reload when there is a change").required(true))
        )
    .subcommand(
        App::new("command-output")
            .about("Perform [cmd] when the [watch] folder (recursively) changes and then reload the image at the path in the last line of the [cmd] output")
            .arg(arg!(-c --cmd [cmd] "The command to execute. The last line of the output provides the path to the image to display.")
            .required(true))
            .arg(watch_arg)
            //.arg(arg!(-w --watch [watch] "The folder to watch for changes").required(true).multiple_occurrences(true))
        )
    .get_matches();

    let mode = match matches.subcommand() {
        Some(("command-image", sub_matches)) => {
            let command: String = sub_matches.value_of_t_or_exit("cmd");
            let watch: Vec<PathBuf> = sub_matches.values_of_t_or_exit("watch");
            let image: PathBuf = sub_matches.value_of_t_or_exit("img");
            Mode::CommandImage {
                command,
                watch,
                image,
            }
        }
        Some(("image", sub_matches)) => {
            let path: PathBuf = sub_matches.value_of_t_or_exit("img");
            Mode::Image { path }
        }
        Some(("command-output", sub_matches)) => {
            let command: String = sub_matches.value_of_t_or_exit("cmd");
            let watch: Vec<PathBuf> = sub_matches.values_of_t_or_exit("watch");
            Mode::CommandOutput { command, watch }
        }
        _ => {
            println!("Please call with --help");
            std::process::exit(0);
        }
    };

    let config = Config::new(mode);
    interface::launch_app(config);
}

pub fn setup_tracing() {
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;

    tracing::info!("Logging initialized");

    let collector = tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout.with_max_level(tracing::Level::DEBUG)));
    tracing::subscriber::set_global_default(collector).expect("Unable to set a global collector");
}
