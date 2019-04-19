extern crate clap;
extern crate colored;

mod command;
mod string_ext;

mod commands;

mod command_chain;
mod git;

use clap::{App, Arg, SubCommand};
use commands::{
    merge,
    on_remote::{self, Remote},
    ship_hotfix, start,
};

fn main() {
    let dry_run = Arg::with_name("dry-run")
        .long("dry-run")
        .help("Don't run stuff, just print what would happen");

    let from_step = Arg::with_name("from-step")
        .long("from-step")
        .takes_value(true)
        .value_name("STEP")
        .help("Start the command from the given step. Used when a previous command failed and needs to be resumed");

    let mut app = App::new("api-git")
        .version(env!("CARGO_PKG_VERSION"))
        .author("David Pedersen <david.pdrsn@gmail.com>")
        .about("Handles common API git operations")
        .subcommand(
            SubCommand::with_name("start")
                .about("Start a new branch")
                .arg(&dry_run)
                .arg(&from_step)
                .arg(Arg::with_name("BRANCH").help("The name of the branch that will be created"))
                .arg(
                    Arg::with_name("push")
                        .long("push")
                        .short("p")
                        .help("Also push the branch and setup tracking with remote branch"),
                ).arg(
                    Arg::with_name("base")
                        .long("base")
                        .short("b")
                        .takes_value(true)
                        .value_name("BASE_BRANCH")
                        .help("Start the branch from here instead of master"),
                ).arg(
                    Arg::with_name("prefix")
                        .long("prefix")
                        .short("x")
                        .help("Prefix the new branch with the name of the base branch. Useful for sprint feature branches that goes into release branches"),
                ),
        ).subcommand(
            SubCommand::with_name("merge")
                .about("Merge one or more branches")
                .arg(&dry_run)
                .arg(&from_step)
                .arg(Arg::with_name("BRANCH").multiple(true).help("The branch(es) that will be merged"))
                .arg(
                    Arg::with_name("into")
                        .long("into")
                        .short("i")
                        .takes_value(true)
                        .value_name("INTO_BRANCH")
                        .help("The branch that will be merged into. Defaults to master"),
                ).arg(
                    Arg::with_name("no-rebase")
                        .long("no-rebase")
                        .help("Just merge directly without rebasing first"),
                )
        ).subcommand(
            SubCommand::with_name("ship-hotfix")
                .about("Merge master into staging and develop and deploy")
                .arg(&dry_run)
                .arg(&from_step)
        ).subcommand(
            SubCommand::with_name("on-staging")
                .about("Merge branch into staging and deploy to staging")
                .arg(Arg::with_name("BRANCH").multiple(false).help("The branch that will be merged"))
                .arg(&dry_run)
                .arg(&from_step)
        ).subcommand(
            SubCommand::with_name("on-develop")
                .about("Merge branch into develop and deploy to develop")
                .arg(Arg::with_name("BRANCH").multiple(false).help("The branch that will be merged"))
                .arg(&dry_run)
                .arg(&from_step)
            );

    let matches = app.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches("start") {
        start::run_start(matches);
    } else if let Some(matches) = matches.subcommand_matches("merge") {
        merge::run_merge(matches);
    } else if let Some(matches) = matches.subcommand_matches("ship-hotfix") {
        ship_hotfix::run_ship_hotfix(matches);
    } else if let Some(matches) = matches.subcommand_matches("on-staging") {
        on_remote::run_on_remote(Remote::Staging, matches);
    } else if let Some(matches) = matches.subcommand_matches("on-develop") {
        on_remote::run_on_remote(Remote::Develop, matches);
    } else {
        app.print_help().expect("failed to print help");
        print!("\n");
        std::process::exit(1);
    }
}
