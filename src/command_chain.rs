use crate::command::Command;
use crate::string_ext::*;
use colored::*;

pub struct CommandChain {
    steps: Vec<Box<dyn Step>>,
}

impl CommandChain {
    pub fn new() -> CommandChain {
        CommandChain { steps: vec![] }
    }

    pub fn add<S>(&mut self, step: S)
    where
        S: 'static + Step,
    {
        self.steps.push(Box::new(step));
    }

    pub fn run_and_print_from_step(&self, idx: usize, rerun_command: &str, runner: &StepRunner) {
        for (step, cmd) in self.steps.iter().enumerate() {
            if step < idx {
                continue;
            }

            println!(
                "{}",
                format!(
                    "-- Running step {}: {}",
                    step.to_string().green(),
                    cmd.as_string(),
                )
                .green(),
            );
            match runner.run_step(&**cmd) {
                RunStepResult::Ok => {}
                RunStepResult::Err => {
                    println!(
                        "{}",
                        format!("Step {} failed. Fix the problem and rerun with:", step).red(),
                    );
                    println!();

                    let path_to_self = "api-git";
                    let retry_command =
                        format!("{} {} --from-step {}", path_to_self, rerun_command, step);
                    println!("{}", retry_command.indent(2));

                    copy_to_clipboard(&retry_command);
                    println!("Retry command has been copied to the clipboard");

                    break;
                }
            }
            println!();
        }
    }
}

fn copy_to_clipboard(text: &str) {
    use clipboard::{ClipboardContext, ClipboardProvider};

    let mut cp = ClipboardContext::new().expect("failed to create clipboard");

    cp.set_contents(text.to_string())
        .expect("failed to copy to clipboard");
}

pub trait Step {
    fn run_step(&self) -> RunStepResult;

    fn as_string(&self) -> String;
}

impl<T> Step for T
where
    T: Command,
{
    fn run_step(&self) -> RunStepResult {
        let output = self.execute();

        if output.status.success() {
            RunStepResult::Ok
        } else {
            RunStepResult::Err
        }
    }

    fn as_string(&self) -> String {
        format!("{} {}", self.command(), self.args().join(" "))
    }
}

pub enum RunStepResult {
    Ok,
    Err,
}

#[allow(dead_code)]
pub enum StepRunner {
    Dry,
    Run,
}

impl StepRunner {
    fn run_step(&self, step: &dyn Step) -> RunStepResult {
        match self {
            StepRunner::Dry => {
                println!("Dry run:");
                println!("{}", step.as_string().indent(2));
                RunStepResult::Ok
            }
            StepRunner::Run => step.run_step(),
        }
    }
}
