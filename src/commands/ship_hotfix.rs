use clap::ArgMatches;
use command_chain::*;
use commands::*;
use git::Git;

pub fn run_ship_hotfix(args: &ArgMatches) {
    ShipHotfixArgs::from_args(&args)
        .unwrap()
        .parse_args_and_run_command(&args, ship_hotfix_command);
}

fn ship_hotfix_command(_args: &ShipHotfixArgs) -> CommandChain {
    let mut c = CommandChain::new();

    c.add(Git::checkout("master"));
    c.add(Git::pull_rebase());
    c.add(Git::push());

    for branch in ["staging", "develop"].iter() {
        c.add(Git::checkout(branch));
        c.add(Git::pull());
        c.add(Git::merge("master"));
        c.add(Git::push());
    }

    c.add(Git::checkout("master"));

    c
}

#[derive(Debug)]
struct ShipHotfixArgs;

impl ShipHotfixArgs {
    fn from_args(_args: &ArgMatches) -> Option<Self> {
        Some(ShipHotfixArgs)
    }
}

impl CommandArgs for ShipHotfixArgs {
    fn rerun_command(&self) -> String {
        let mut rerun_command = String::new();
        rerun_command.push_str("ship-hotfix");
        rerun_command
    }
}
