pub mod merge;
pub mod on_staging;
pub mod ship_hotfix;
pub mod start;

use clap::ArgMatches;
use command_chain::*;
use std::process;

pub trait CommandArgs: Sized {
    fn from_args(args: &ArgMatches) -> Option<Self>;

    fn rerun_command(&self) -> String;

    fn parse_args_and_run_command<F>(args: &ArgMatches, command: F)
    where
        F: Fn(&Self) -> CommandChain,
    {
        let cmd_args = match Self::from_args(&args) {
            Some(cmd_args) => cmd_args,
            _ => {
                println!("Invalid args given to command");
                process::exit(1);
            }
        };

        let from_step: usize = args
            .value_of("from-step")
            .and_then(|step| {
                let step: Option<usize> = step.parse().ok();
                step.map(|s| s + 1)
            })
            .unwrap_or(0);

        let mut rerun_command = cmd_args.rerun_command();
        let dry_run = args.is_present("dry-run");

        if dry_run {
            rerun_command = format!("{cmd} --dry-run", cmd = rerun_command);
        }

        let step_runner = if dry_run {
            StepRunner::Dry
        } else {
            StepRunner::Run
        };

        command(&cmd_args).run_and_print_from_step(from_step, &rerun_command, &step_runner);
    }
}
