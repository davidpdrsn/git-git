#![allow(dead_code, unused_imports, unused_variables)]

mod command;
use command::Command;

mod git;
use git::Git;

mod command_chain;
use command_chain::*;

fn main() {
    let chain = CommandChain::new()
        .add(Git::status())
        .add(Git::log())
        .add(Git::branch());

    chain.run();
}
