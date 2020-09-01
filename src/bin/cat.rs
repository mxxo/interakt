use interakt::interact;
use std::convert::identity;

fn main() -> std::io::Result<()> { interact(identity) }
