# brain storm bike shed

## salmon migration

- [ ] replace uses of `regex` with `syn`:
  - [ ] to identify the docstring with a possible manifest
  - [ ] to detect the `main` function, or detect top-level statements that
        aren't valid outside of a function, if there is no main.
  - [ ] keep `regex` for detecting header comments (which won't be in the AST)

## high noise low filter

- [x] drop support for `.ers` extension; only support `.rs`
- [x] rename the binary to `rust` (deviating from package name)
- [ ] remove template feature, replacing current templates with uses of `quote!`
      and similar.
  - `--expr` will remain
  - `--loop`, and `--loop --count` will be considered
- [ ] pick up implicit dependencies
  - any `::root` paths are implicit dependencies
  - paths-like syntax in macros calls like like `println!(::foo::bar())` aren't
    considered paths by `syn`. do we still want them?
- [ ] automatically version non-versioned dependencies using the latest version
      at the last modified timestamp of the script.
  - what do we do for yanked versions?
- [ ] pick up declared modules
  - [ ] support `#[path]` annotations
- [ ] support `[toolchain]` items in manifest, but put them in
      `rust-toolchain.toml` instead of `Cargo.toml`
- [ ] add a second binary named something more like `rust-eval`, which naively
      takes its command-line arguments, joins them with spaces, evaluates the
      result, and prints it unless it's of the unit type. or have some really
      convenient shortcut, like `rust eval 2 + 2` or `rust -- 2 + 2`.
- [ ] maybe require explicit path qualification for filenames that don't have
      any `/` or `.`
- [ ] use git's hashing scheme for blobs/trees.
- [ ] export to playground, inlining external modules.
- [ ] the model is `python` and `deno`
- [ ] replace any use of (non-doc) comments with something that's visible in
      Rust's AST
- [ ] use the local Cargo.toml if one exists
- [ ] prioritize, upstream
