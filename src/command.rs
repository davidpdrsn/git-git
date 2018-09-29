use std::process;

pub trait Command {
    fn command(&self) -> String;

    fn args(&self) -> Vec<String>;

    fn execute(&self) -> Output {
        let mut cmd = process::Command::new(self.command());

        for arg in self.args() {
            cmd.arg(arg);
        }

        let output = cmd.output().expect("failed to execute process");

        Output {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            status: output.status,
        }
    }
}

pub struct Output {
    pub status: process::ExitStatus,
    pub stdout: String,
    pub stderr: String,
}
