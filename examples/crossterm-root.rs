#!/usr/bin/env rust
//! [@]: stable-2021-10-21
//! [@]: 13654361113843.17134

// ^ support toolchain AND/OR mtime override as top-level [@] link references
// but we don't actually use the two of them together, right? I don't know
// whether we have a full index of releases to be able to figure out "the latest
// stable" based on a given mtime, and probably don't want to. although for nightly...

//! [@]: 13654361113843.17134 (beta)

/// [@]: ~2.1.4 (-defaults +std)
use serde;

static __PACKAGE__: &str = r#"
[package]
edition = 2021
"#;
static __TOOLCHAIN__: &str = "nightly";

// Look in git to find modification times for files, if we're in git?


fn main() {
    let mut s = String::new();
    s.push_str(&::crossterm::style::Stylize::red("hello").to_string());
    s.push_str(" ");
    s.push_str(&::crossterm::style::Stylize::blue("hello").to_string());

    println!("{s}");
}
