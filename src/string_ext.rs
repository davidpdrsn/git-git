pub trait Indent {
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
