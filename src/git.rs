use command::Command;

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
