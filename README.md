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
xtaskops = "0.2.2"
```

For most recent version see [crates.io](https://crates.io/crates/xtaskops)


## Usage

You should have the `xtask` concept already set up for your project. 

* To get started quickly, you can use [this Rust CI starter](https://github.com/rusty-ferris-club/rust-starter)
* To set up manually [follow the repo here](https://github.com/matklad/cargo-xtask/tree/master/examples/hello-world).

## Available Tasks

Full workflow tasks for your daily development.

* **bloat_deps**	Show biggest crates in release build
* **bloat_time**	Show crate build times
* **dev**	Run `cargo check` followed by `cargo test` for every file change
* **ci**	Run typical CI tasks in series: fmt, clippy, and tests
* **coverage**	Run coverage
* **docs**	Run cargo docs in watch mode
* **install**	Instal cargo tools
* **powerset**	Perform a CI build with powerset of features

Here's an example for how to integrate the **coverage** task with `clap`:

```rust
use xtaskops::tasks;
// assuming you use `clap`
let res = match matches.subcommand() {
    Some(("coverage", sm)) => tasks::coverage(sm.is_present("dev")),
  //..
```

## Quick start

You can include everything from `xtask` in your project. In your `xtask/main.rs`:

```rust
fn main() -> Result<(), anyhow::Error> {
    xtaskops::tasks::main()
}
```



### Ops

Low level convenience operations, for file system operations, user input and more.

```rust
use xtaskops::ops::{remove_dir, create_dir_all, cmd};

remove_dir("target")?;
create_dir_all("target")?;
// cmd! is from the `duct` library
cmd!("cargo", "watch", "-s", "cargo doc --no-deps").run()?;
Ok(())
```

## Running Tasks

Run:

```
$ cargo xtask coverage
```

Recommended: alias `cargo xtask` to `x`:

```bash
# in your zshrc/shell rcfile
alias x="cargo xtask"
```

# Copyright

Copyright (c) 2022 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
