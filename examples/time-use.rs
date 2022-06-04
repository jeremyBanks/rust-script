#!/usr/bin/env rust
use time;

fn main() {
    let s = time::Instant::now().rfc822z().to_string();
    println!("{s}");
}
