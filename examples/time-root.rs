#!/usr/bin/env rust
use ::crossterm::style::Stylize;
use crossterm::style::Stylize;

fn main() {
    let s: String = [
        "hello".red(),
        " ",
        "world".blue()
    ].collect();
    println!("{s}");
}
