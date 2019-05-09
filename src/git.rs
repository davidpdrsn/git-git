use crate::command::Command;
use git2::{BranchType, Repository};

#[derive(Debug)]
pub struct Git {
    command: String,
    args: Vec<String>,
}

#[allow(dead_code)]
impl Git {
    pub fn status() -> Self {
        Git::from("status")
    }

    pub fn log() -> Self {
        Git::from("log")
    }

    pub fn branch(branch: &str) -> Self {
        Git::from(format!("branch {}", branch))
    }

    pub fn pull() -> Self {
        Git::from("pull")
    }

    pub fn pull_rebase() -> Self {
        Git::from("pull --rebase")
    }

    pub fn push() -> Self {
        Git::from("push")
    }

    pub fn push_staging() -> Self {
        Git::from("push staging staging:master")
    }

    pub fn push_develop() -> Self {
        Git::from("push development develop:master")
    }

    pub fn push_and_set_upstream(branch: &str) -> Self {
        Git::from(format!("push --set-upstream origin {}", branch))
    }

    pub fn force_push() -> Self {
        Git::from("push -f")
    }

    pub fn rebase(branch: &str) -> Self {
        Git::from(format!("rebase {}", branch))
    }

    pub fn checkout(branch: &str) -> Self {
        Git::from(format!("checkout {}", branch))
    }

    pub fn merge(branch: &str) -> Self {
        Git::from(format!("merge --no-edit {}", branch))
    }

    pub fn fast_forward_merge(branch: &str) -> Self {
        Git::from(format!("merge --ff-only {}", branch))
    }

    pub fn delete_branch(branch: &str) -> Self {
        Git::from(format!("branch -D {}", branch))
    }

    pub fn delete_remote_branch(branch: &str) -> Self {
        Git::from(format!("push origin :{}", branch))
    }

    pub fn prune_remote() -> Self {
        Git::from("fetch origin --prune")
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

impl From<String> for Git {
    fn from(s: String) -> Git {
        Git::from(s.as_ref())
    }
}

/// Returns if a branch with the given name exists.
pub fn branch_exists(needle: &str) -> bool {
    let repo = open_repo();

    let mut branches = repo
        .branches(Some(BranchType::Local))
        .expect("get branches");

    branches.any(|branch| {
        let (branch, _branch_type) = branch.expect("get branch");
        let name = branch.name().expect("branch name");
        name == Some(needle)
    })
}

/// Get the name of the current branch
pub fn current_branch() -> String {
    let repo = open_repo();

    let head = repo.head().expect("failed to get HEAD");

    let branch = repo
        .branches(Some(BranchType::Local))
        .expect("get branches")
        .map(|branch| branch.unwrap().0)
        .find(|branch| branch.get() == &head)
        .expect("failed to branch HEAD is pointing at");

    branch
        .name()
        .expect("failed to get branch name")
        .expect("branch name isn't valid UTF-8")
        .to_string()
}

fn open_repo() -> Repository {
    Repository::open(".").expect("failed to open repo in current directory")
}

pub fn current_branch_with_confirm(
    question: impl Fn(&str) -> String,
    default: ConfirmDefault,
) -> String {
    let current_branch = current_branch();

    if confirm(&question(&current_branch), default) {
        current_branch
    } else {
        std::process::exit(0)
    }
}

fn confirm(question: &str, default: ConfirmDefault) -> bool {
    use std::io::{self, Read, Write};

    match default {
        ConfirmDefault::Yes => {
            print!("{}? Y/n ", question);
        }
        ConfirmDefault::No => {
            print!("{}? y/N ", question);
        }
    }
    io::stdout().flush().expect("failed to flush stdout");

    let input = io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as char)
        .expect("failed to read from stdin");

    if input == '\n' {
        match default {
            ConfirmDefault::Yes => return true,
            ConfirmDefault::No => return false,
        }
    } else {
        if input == 'y' {
            return true;
        }

        if input == 'n' {
            return false;
        }
    }

    eprintln!("Invalid answer {:?}", input);
    std::process::exit(1)
}

#[derive(Debug, Copy, Clone)]
pub enum ConfirmDefault {
    Yes,
    No,
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_branch_exists() {
        assert!(branch_exists("master"));
        assert!(!branch_exists("doesnt-exist"));
    }
}
