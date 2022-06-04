# brain storm bike shed

## high noise low filter

- [x] drop support for `.ers` extension; only support `.rs`
- [x] rename the binary to `rust` (deviating from package name)
- [ ] replace uses of `regex` with `syn`:
  - [ ] to identify the docstring with a possible manifest
  - [ ] to detect the `main` function, or detect top-level statements that
        aren't valid outside of a function, if there is no main.
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
      convenient shortcut, like `rust eval`
- [ ] use git's hashing scheme for blobs/trees.
- [ ] export to playground, inlining external modules.
