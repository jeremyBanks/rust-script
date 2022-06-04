#!/usr/bin/env rust
fn main() {
    let mut s = String::new();
    s.push_str(&::crossterm::style::Stylize::red("hello").to_string());
    s.push_str(" ");
    s.push_str(&::crossterm::style::Stylize::blue("hello").to_string());

    println!("{s}");
}
