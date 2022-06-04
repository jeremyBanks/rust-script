#!/usr/bin/env rust
/// This is a regular crate doc comment, but it also contains a partial
/// Cargo manifest.  Note the use of a *fenced* code block, and the
/// `cargo` "language".
///
/// ```cargo
/// [dependencies]
/// time = "0.1.25"
/// ```
use time::now;

println!("{}", now().rfc822z());
