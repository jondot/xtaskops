//!
//! Complete xtask tasks such as `docs`, `ci` and others
//!
use crate::ops::{clean_files, confirm, remove_dir};
use anyhow::Result as AnyResult;
use derive_builder::Builder;
use duct::cmd;
use std::fs::create_dir_all;

///
/// Run cargo docs in watch mode
///
/// # Errors
/// Fails if any command fails
///
pub fn docs() -> AnyResult<()> {
    cmd!("cargo", "watch", "-s", "cargo doc --no-deps").run()?;
    Ok(())
}

/// Build a CI run
#[derive(Builder)]
#[builder(setter(into))]
pub struct CI {
    /// run with nightly
    /// default: on
    #[builder(default = "false")]
    pub nightly: bool,

    /// turn all clippy lints on: pedantic, nursery, 2018-idioms
    /// default: on
    #[builder(default = "true")]
    pub clippy_max: bool,
}

impl CIBuilder {
    /// Runs this builder
    ///
    /// # Errors
    ///
    /// This function will return an error if run failed
    pub fn run(&self) -> AnyResult<()> {
        let t = self.build()?;
        let mut check_args = vec!["fmt", "--all", "--", "--check"];
        if t.nightly {
            check_args.insert(0, "+nightly");
        }

        let mut clippy_args = vec!["clippy", "--", "-D", "warnings"];
        if t.clippy_max {
            clippy_args.extend([
                "-W",
                "clippy::pedantic",
                "-W",
                "clippy::nursery",
                "-W",
                "rust-2018-idioms",
            ]);
        }

        cmd("cargo", check_args.as_slice()).run()?;
        cmd("cargo", clippy_args.as_slice()).run()?;
        cmd!("cargo", "test").run()?;
        cmd!("cargo", "test", "--doc").run()?;
        Ok(())
    }
}

///
/// Run typical CI tasks in series: `fmt`, `clippy`, and tests
///
/// # Errors
/// Fails if any command fails
///
pub fn ci() -> AnyResult<()> {
    CIBuilder::default().run()
}

///
/// Run coverage
///
/// # Errors
/// Fails if any command fails
///
pub fn coverage(devmode: bool) -> AnyResult<()> {
    remove_dir("coverage")?;
    create_dir_all("coverage")?;

    println!("=== running coverage ===");
    cmd!("cargo", "test")
        .env("CARGO_INCREMENTAL", "0")
        .env("RUSTFLAGS", "-Cinstrument-coverage")
        .env("LLVM_PROFILE_FILE", "cargo-test-%p-%m.profraw")
        .run()?;
    println!("ok.");

    println!("=== generating report ===");
    let (fmt, file) = if devmode {
        ("html", "coverage/html")
    } else {
        ("lcov", "coverage/tests.lcov")
    };
    cmd!(
        "grcov",
        ".",
        "--binary-path",
        "./target/debug/deps",
        "-s",
        ".",
        "-t",
        fmt,
        "--branch",
        "--ignore-not-existing",
        "--ignore",
        "../*",
        "--ignore",
        "/*",
        "--ignore",
        "xtask/*",
        "--ignore",
        "*/src/tests/*",
        "-o",
        file,
    )
    .run()?;
    println!("ok.");

    println!("=== cleaning up ===");
    clean_files("**/*.profraw")?;
    println!("ok.");
    if devmode {
        if confirm("open report folder?") {
            cmd!("open", file).run()?;
        } else {
            println!("report location: {}", file);
        }
    }

    Ok(())
}

/// Build a powerset test
#[derive(Builder)]
#[builder(setter(into))]
pub struct Powerset {
    /// powerset depth
    #[builder(default = "2")]
    pub depth: i32,

    /// dont run with no feature at all
    #[builder(default = "false")]
    pub exclude_no_default_features: bool,
}

impl PowersetBuilder {
    /// Builds and runs a powerset test
    ///
    /// # Errors
    ///
    /// This function will return an error if run failed
    pub fn run(&self) -> AnyResult<()> {
        let t = self.build()?;
        let depth = format!("{}", t.depth);
        let mut common = vec![
            "--workspace",
            "--exclude",
            "xtask",
            "--feature-powerset",
            "--depth",
            &depth,
        ];
        if t.exclude_no_default_features {
            common.push("--exclude-no-default-features");
        }
        cmd(
            "cargo",
            &[
                &["hack", "clippy"],
                common.as_slice(),
                &["--", "-D", "warnings"],
            ]
            .concat(),
        )
        .run()?;
        cmd("cargo", &[&["hack"], common.as_slice(), &["test"]].concat()).run()?;
        cmd(
            "cargo",
            &[&["hack", "test"], common.as_slice(), &["--doc"]].concat(),
        )
        .run()?;
        Ok(())
    }
}

///
/// Perform a CI build with powerset of features
///
/// # Errors
/// Errors if one of the commands failed
///
pub fn powerset() -> AnyResult<()> {
    PowersetBuilder::default().run()
}

///
/// Show biggest crates in release build
///
/// # Errors
/// Errors if the command failed
///
pub fn bloat_deps() -> AnyResult<()> {
    cmd!("cargo", "bloat", "--release", "--crates").run()?;
    Ok(())
}

///
/// Show crate build times
///
/// # Errors
/// Errors if the command failed
///
pub fn bloat_time() -> AnyResult<()> {
    cmd!("cargo", "bloat", "--time", "-j", "1").run()?;
    Ok(())
}

///
/// Watch changes and after every change: `cargo check`, followed by `cargo test`
/// If `cargo check` fails, tests will not run.
///
/// # Errors
/// Errors if the command failed
///
pub fn dev() -> AnyResult<()> {
    cmd!("cargo", "watch", "-x", "check", "-x", "test").run()?;
    Ok(())
}

///
/// Instal cargo tools
///
/// # Errors
/// Errors if one of the commands failed
///
pub fn install() -> AnyResult<()> {
    cmd!("cargo", "install", "cargo-watch").run()?;
    cmd!("cargo", "install", "cargo-hack").run()?;
    cmd!("cargo", "install", "cargo-bloat").run()?;
    Ok(())
}

/// Set up a main for your xtask. Uses clap.
/// To customize, look at this function's source and copy it to your
/// own xtask project.
///
/// # Errors
///
/// This function will return an error if any command failed
pub fn main() -> AnyResult<()> {
    use clap::{AppSettings, Arg, Command};
    let cli = Command::new("xtask")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            Command::new("coverage").arg(
                Arg::new("dev")
                    .short('d')
                    .long("dev")
                    .help("generate an html report")
                    .takes_value(false),
            ),
        )
        .subcommand(Command::new("vars"))
        .subcommand(Command::new("ci"))
        .subcommand(Command::new("powerset"))
        .subcommand(Command::new("bloat-deps"))
        .subcommand(Command::new("bloat-time"))
        .subcommand(Command::new("docs"));
    let matches = cli.get_matches();

    let root = crate::ops::root_dir();
    let res = match matches.subcommand() {
        Some(("coverage", sm)) => crate::tasks::coverage(sm.is_present("dev")),
        Some(("vars", _)) => {
            println!("root: {:?}", root);
            Ok(())
        }
        Some(("ci", _)) => crate::tasks::ci(),
        Some(("docs", _)) => crate::tasks::docs(),
        Some(("powerset", _)) => crate::tasks::powerset(),
        Some(("bloat-deps", _)) => crate::tasks::bloat_deps(),
        Some(("bloat-time", _)) => crate::tasks::bloat_time(),
        _ => unreachable!("unreachable branch"),
    };
    res
}
