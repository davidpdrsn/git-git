pub mod merge;
pub mod ship_hotfix;
pub mod start;

use clap::ArgMatches;
use command_chain::*;
use std::process;

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

    fn from_step(&self) -> usize;

    fn parse_args_and_run_command<F>(args: &ArgMatches, command: F)
    where
        F: Fn(&Self) -> CommandChain,
    {
        let args = if let Some(args) = Self::from_args(&args) {
            args
        } else {
            println!("Invalid args given to command");
            process::exit(1);
        };

        command(&args).run_and_print_from_step(
            args.from_step(),
            &args.rerun_command(),
            &args.step_runner(),
        );
    }
}
