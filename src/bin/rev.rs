use interakt::interact;
fn main() -> std::io::Result<()> {
    let rev_lines = |s: String| -> String {
        s.lines()
         .map(|line| line.chars().rev().collect())
         .collect::<Vec<String>>()
         .join("\n")
    };
    interact(rev_lines)
}
