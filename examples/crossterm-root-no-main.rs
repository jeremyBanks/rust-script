#!/usr/bin/env rust

let mut s = String::new();
println!("lol;");
s.push_str(&::crossterm::style::Stylize::red("hello").to_string());
s.push_str(" ");
s.push_str(&::crossterm::style::Stylize::blue("hello").to_string());
