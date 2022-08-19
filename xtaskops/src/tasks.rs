//!
//! Complete xtask tasks such as `docs`, `ci` and others
//!
use crate::ops::{clean_files, confirm, remove_dir};
use anyhow::Result as AnyResult;
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

///
/// Run typical CI tasks in series: `fmt`, `clippy`, and tests
///
/// # Errors
/// Fails if any command fails
///
pub fn ci() -> AnyResult<()> {
    cmd!("cargo", "+nightly", "fmt", "--all", "--", "--check").run()?;
    cmd!("cargo", "clippy", "--", "-D", "warnings").run()?;
    cmd!("cargo", "test").run()?;
    cmd!("cargo", "test", "--doc").run()?;
    Ok(())
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

///
/// Perform a CI build with powerset of features
///
/// # Errors
/// Errors if one of the commands failed
///
pub fn powerset() -> AnyResult<()> {
    let common = &[
        "--workspace",
        "--exclude",
        "xtask",
        "--feature-powerset",
        "--depth",
        "2",
    ];
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
