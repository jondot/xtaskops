xtaskops
========

[<img alt="github" src="https://img.shields.io/badge/github-jondot/xtaskops-8dagcb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/jondot/xtaskops)
[<img alt="crates.io" src="https://img.shields.io/crates/v/xtaskops.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/xtaskops)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-xtaskops-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/xtaskops)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/jondot/xtaskops/Build/master?style=for-the-badge" height="20">](https://github.com/jondot/xtaskops/actions?query=branch%3Amaster)

This is a Rust library that has a few goodies for working with the `xtask` concept.

## Dependency

```toml
[dependencies]
xtaskops = "0.1.0"
```

For most recent version see [crates.io](https://crates.io/crates/xtaskops)


## Usage

If you've set up a local `xtask` folder, and are building you tasks, you can use `xtaskops`.

### Tasks

Coverage:

```rust
use xtaskops::tasks;
// assuming you use `clap`
let res = match matches.subcommand() {
    Some(("coverage", sm)) => tasks::coverage(sm.is_present("dev")),
  //..
```

Run:

```
$ cargo xtask coverage
```

Recommended: alias `cargo xtask` to `x`:

```bash
# in your zshrc/shell rcfile
alias x="cargo xtask"
```

### Ops

```rust
use xtaskops::ops::{remove_dir, create_dir_all, cmd};

remove_dir("target")?;
create_dir_all("target")?;
// cmd! is from the `duct` library
cmd!("cargo", "watch", "-s", "cargo doc --no-deps").run()?;
Ok(())
```

# Copyright

Copyright (c) 2022 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
