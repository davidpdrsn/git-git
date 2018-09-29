use command::Command;

pub struct CommandChain {
    steps: Vec<Box<Step>>,
}

impl CommandChain {
    pub fn new() -> Self {
        CommandChain { steps: vec![] }
    }

    pub fn add<S>(mut self, step: S) -> Self
    where
        S: 'static + Step,
    {
        self.steps.push(Box::new(step));
        self
    }

    pub fn run(&self) {
        self.run_from_step(1);
    }

    pub fn run_from_step(&self, idx: usize) {
        let idx = idx - 1;

        for (step, cmd) in self.steps.iter().enumerate() {
            if step < idx {
                continue;
            }

            println!("-- Running step {}: {}", step, cmd.as_string());
            match cmd.run_step() {
                RunResult::Ok(output) => {
                    println!("{}", output);
                }
                RunResult::Err(output) => {
                    println!(
                        "Step {} failed. Fix the problem and rerun from step {}",
                        step, step
                    );
                    println!("Stderr of failed command:");
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
    fn run_step(&self) -> RunResult;

    fn as_string(&self) -> String;
}

impl<T> Step for T
where
    T: Command,
{
    fn run_step(&self) -> RunResult {
        let output = self.execute();

        if output.status.success() {
            RunResult::Ok(output.stdout)
        } else {
            RunResult::Err(output.stderr)
        }
    }

    fn as_string(&self) -> String {
        format!("{} {}", self.command(), self.args().join(" "))
    }
}

pub enum RunResult {
    Ok(String),
    Err(String),
}

trait Indent {
    fn indent(&self, n: u32) -> Self;
}

impl Indent for String {
    fn indent(&self, n: u32) -> Self {
        let mut indent = String::new();
        for _ in 0..n {
            indent.push_str(" ");
        }

        self.lines()
            .map(|line| format!("{}{}", indent, line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
