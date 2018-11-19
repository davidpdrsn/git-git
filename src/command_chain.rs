use colored::*;
use command::Command;
use string_ext::*;

pub struct CommandChain {
    steps: Vec<Box<Step>>,
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
            match runner.run_step(cmd) {
                RunStepResult::Ok(output) => {
                    println!("{}", output);
                }
                RunStepResult::Err(output) => {
                    println!(
                        "{}",
                        format!("Step {} failed. Fix the problem and rerun with:", step).red(),
                    );
                    println!("");
                    let path_to_self = "api-git";
                    println!(
                        "{}",
                        format!("{} {} --from-step {}", path_to_self, rerun_command, step)
                            .indent(2),
                    );
                    println!("");
                    println!("{}", format!("Stderr of failed command:").red());
                    println!("");
                    println!("{}", output.indent(2));
                    break;
                }
            }
            println!("");
        }
    }
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
            RunStepResult::Ok(output.stdout)
        } else {
            RunStepResult::Err(output.stderr)
        }
    }

    fn as_string(&self) -> String {
        format!("{} {}", self.command(), self.args().join(" "))
    }
}

pub enum RunStepResult {
    Ok(String),
    Err(String),
}

#[allow(dead_code)]
pub enum StepRunner {
    Dry,
    Run,
}

impl StepRunner {
    fn run_step(&self, step: &Box<Step>) -> RunStepResult {
        match self {
            StepRunner::Dry => {
                println!("Dry run:");
                println!("{}", step.as_string().indent(2));
                RunStepResult::Ok(String::new())
            }
            StepRunner::Run => step.run_step(),
        }
    }
}
