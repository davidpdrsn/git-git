use command::Command;
use command_chain::{RunResult, Step};

#[derive(Debug)]
pub struct Git {
    command: String,
    args: Vec<String>,
}

impl Git {
    pub fn status() -> Self {
        Git::from("status")
    }

    pub fn log() -> Self {
        Git::from("log")
    }

    pub fn branch() -> Self {
        Git::from("branch")
    }
}

impl Command for Git {
    fn command(&self) -> String {
        self.command.clone()
    }

    fn args(&self) -> Vec<String> {
        self.args.clone()
    }
}

impl<'a> From<&'a str> for Git {
    fn from(s: &'a str) -> Git {
        let args = s.split(" ").map(|s| s.into()).collect();

        Git {
            command: String::from("git"),
            args,
        }
    }
}
