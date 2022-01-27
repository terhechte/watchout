use crate::config::Config;
use crate::runner::Runner;
use crate::WatchoutState;

use crate::{SELECTOR_COMMAND, SELECTOR_EVENT};

use druid::{AppDelegate, Command, DelegateCtx, Env, ExtEventSink, Handled, Target};

pub struct Delegate {
    config: Config,
    sink: ExtEventSink,
    runner: Runner,
}

impl Delegate {
    pub fn new(sink: ExtEventSink, config: Config) -> Delegate {
        let runner = Runner {
            config: config.clone(),
        };
        Delegate {
            config,
            sink: sink.clone(),
            runner,
        }
    }
}

impl AppDelegate<WatchoutState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut WatchoutState,
        _env: &Env,
    ) -> Handled {
        if let Some(_) = cmd.get(druid::commands::CLOSE_WINDOW) {
            std::process::exit(0);
        }
        if let Some(paths) = cmd.get(SELECTOR_EVENT) {
            tracing::info!("Detected path change: {:?}", &paths);
            data.loading = true;
            self.runner.run(self.sink.clone());
            return Handled::Yes;
        }
        if let Some(result) = cmd.get(SELECTOR_COMMAND) {
            data.loading = false;
            data.failure = false;
            match result.take() {
                Some(Ok(n)) => {
                    tracing::info!("Updated Image ({} x {})", n.width(), n.height());
                    data.image = n;
                }
                Some(Err(e)) => {
                    tracing::error!("Error: {:?}", e);
                    // Also print the error
                    println!("{:?}", e);
                    data.failure = true;
                }
                None => (),
            }
            return Handled::Yes;
        }
        Handled::No
    }

    fn window_added(
        &mut self,
        _id: druid::WindowId,
        _data: &mut WatchoutState,
        _env: &Env,
        ctx: &mut DelegateCtx,
    ) {
        tracing::info!("Trigger first reload");
        // trigger the first reload
        let paths = self.config.watch_paths();
        let command = druid::Command::new(SELECTOR_EVENT, paths, druid::Target::Auto);
        ctx.submit_command(command)
    }
}
