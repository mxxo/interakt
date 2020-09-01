//! Provides the higher order `IO` function [`interact`](crate::interact) for prototyping command line interfaces.
//!
//! *A small tutorial package for Haskell refugees adapted from this lovely [walkthrough](https://wiki.haskell.org/Tutorials/Programming_Haskell/String_IO#IO).*
//!
//! Interact takes a function of type `Fn(String) -> String`, runs it on `stdin`
//! and prints the result to `stdout`.
//!
//! # Examples
//! ## `cat`
//! ```
//! use interakt::interact;
//! use std::convert::identity;
//!
//! fn main() -> std::io::Result<()> { interact(identity) }
//! ```
//! ```shell
//! $ cat cat.rs | cargo run --bin cat
//! use interakt::interact;
//! use std::convert::identity;
//!
//! fn main() -> std::io::Result<()> { interact(identity) }
//! ```
//! ## `wc`
//! ```
//! # use interakt::interact;
//! fn main() -> std::io::Result<()> {
//!     let count = |s: String| -> String { format!("{}\n", s.len()) };
//!     interact(count)
//! }
//! ```
//! ```shell
//! $ cat cat.rs | cargo run --bin wc
//! 109
//! ```
//! ## `wc -l`
//! ```
//! # use interakt::interact;
//! fn main() -> std::io::Result<()> {
//!     let count_lines = |s: String| -> String {
//!         format!("{}\n", s.lines().collect::<Vec<_>>().len())
//!     };
//!     interact(count_lines)
//! }
//! ```
//! ```shell
//! $ cat cat.rs | cargo run --bin wcl
//! 4
//! ```
//! ## `rev`
//! ```
//! # use interakt::interact;
//! fn main() -> std::io::Result<()> {
//!     let rev_lines = |s: String| -> String {
//!         s.lines()
//!          .map(|line| line.chars().rev().collect())
//!          .collect::<Vec<String>>()
//!          .join("\n")
//!     };
//!     interact(rev_lines)
//! }
//! ```
//! ```text
//! $ cat cat.rs | cargo run --bin rev
//! ;tcaretni::tkaretni esu
//! ;ytitnedi::trevnoc::dts esu
//!
//! } )ytitnedi(tcaretni { >)(<tluseR::oi::dts >- )(niam nf
//! ```
use std::io::{Read, Write};

/// The higher-order `interact` function.
///
/// In Haskell, the interact function is:
/// ```text
/// interact f = do s <- getContents
///                 putStr (f s)
/// ```
pub fn interact<F>(f: F) -> std::io::Result<()>
    where F: std::ops::Fn(String) -> String
{
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    std::io::stdout().write_all(f(buffer).as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
