pub mod start;
pub mod merge;

use clap::{ArgMatches};
use command_chain::*;

pub trait CommandArgs: Sized {
    fn from_args(args: &ArgMatches) -> Option<Self>;

    fn dry_run(&self) -> bool;

    fn rerun_command(&self) -> String;

    fn step_runner(&self) -> StepRunner {
        if self.dry_run() {
            StepRunner::Dry
        } else {
            StepRunner::Run
        }
    }
}
