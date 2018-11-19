pub mod merge;
pub mod on_remote;
pub mod ship_hotfix;
pub mod start;

use clap::ArgMatches;
use command_chain::*;
use std::process;

pub trait CommandArgs
where
    Self: Sized,
{
    fn rerun_command(&self) -> String;

    fn parse_args_and_run_command<F>(&self, args: &ArgMatches, command: F)
    where
        F: Fn(&Self) -> CommandChain,
    {
        let from_step: usize = args
            .value_of("from-step")
            .and_then(|step| {
                let step: Option<usize> = step.parse().ok();
                step.map(|s| s + 1)
            })
            .unwrap_or(0);

        let mut rerun_command = self.rerun_command();
        let dry_run = args.is_present("dry-run");

        if dry_run {
            rerun_command = format!("{cmd} --dry-run", cmd = rerun_command);
        }

        let step_runner = if dry_run {
            StepRunner::Dry
        } else {
            StepRunner::Run
        };

        command(&self).run_and_print_from_step(from_step, &rerun_command, &step_runner);
    }
}
