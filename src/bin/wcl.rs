// wc -l
use interakt::interact;
fn main() -> std::io::Result<()> {
    let count_lines = |s: String| -> String {
        format!("{}\n", s.lines().collect::<Vec<_>>().len())
    };
    interact(count_lines)
}
