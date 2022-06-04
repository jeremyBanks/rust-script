#!/usr/bin/env rust

fn main() {
    let s = ::time::now().rfc822z().to_string();
    println!("{s}");
}
