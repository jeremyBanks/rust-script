#!/usr/bin/env rust-script
/*!
This is a regular crate doc comment, but it also contains a partial
Cargo manifest.  Note the use of a *fenced* code block, and the
`cargo` "language".

```cargo
[dependencies]
time = "0.1.25"
```
*/
fn main() {
    let s = ::time::now().rfc822z().to_string();
    println!("{s}");
}
