#!/usr/bin/env rust
use {{{::{{{{crossterm::style::{{{{Stylize}}}}}}}}}}};

fn main() {
    let mut s = std::string::String::new();
    s.push_str(&"hello".red().to_string());
    s.push_str(" ");
    s.push_str(&"hello".blue().to_string());

    println!("{s}");
}
