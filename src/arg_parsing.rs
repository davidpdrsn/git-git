use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "api-git", about = "Utility for doing common git operations")]
pub enum Opt {
    /// Start a new branch
    #[structopt(name = "start")]
    Start(Start),
}

#[derive(StructOpt, Debug)]
pub struct Start {
    /// The name of the branch that will be created
    pub branch: String,

    /// Don't run stuff, just print what would happen
    #[structopt(long = "dry-run")]
    pub dry_run: bool,

    /// Start the command from the given step. Used when a previous command failed and needs to be resumed
    #[structopt(long = "dry-run")]
    pub from_step: bool,
}
