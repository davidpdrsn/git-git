use std::process;

pub trait Command {
    fn command(&self) -> String;

    fn args(&self) -> Vec<String>;

    fn execute(&self) -> Output {
        let mut cmd = process::Command::new(self.command());

        for arg in self.args() {
            cmd.arg(arg);
        }

        let status = cmd.status().expect("failed to execute process");

        Output { status }
    }
}

pub struct Output {
    pub status: process::ExitStatus,
}
