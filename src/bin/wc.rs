// wc
use interakt::interact;
fn main() -> std::io::Result<()> {
    let count = |s: String| -> String { format!("{}\n", s.len()) };
    interact(count)
}
