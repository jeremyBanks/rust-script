#!/usr/bin/env rust
use time;

fn main() {
    let s = time::now().rfc822z().to_string();
    println!("{s}");
}
