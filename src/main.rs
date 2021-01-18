mod config;
mod version;

use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;
use version::{Bump, Version};

fn main() {
    let args: Vec<String> = env::args().collect();

    // common flags
    let pre_release_flag = Flag::new("pre", FlagType::String).description("Use a pre-release tag.");
    let config_file_flag = Flag::new("config", FlagType::String)
        .description("Select config file to use. Default is `Config.toml`.");

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cargo-semver")
        .action(default_action)
        .flag(config_file_flag.clone())
        .command(
            Command::new("set")
                .description("Set the version in Cargo.toml")
                .usage("cargo-semver set [VERSION]")
                .action(set_version_action)
                .flag(config_file_flag.clone()),
        )
        .command(
            Command::new("patch")
                .description("Increments the patch version number in Cargo.toml")
                .usage("cargo-semver patch")
                .action(patch_action)
                .flag(pre_release_flag.clone())
                .flag(config_file_flag.clone()),
        )
        .command(
            Command::new("minor")
                .description("Increments the minor version number in Cargo.toml")
                .usage("cargo-semver minor")
                .action(minor_action)
                .flag(pre_release_flag.clone())
                .flag(config_file_flag.clone()),
        )
        .command(
            Command::new("major")
                .description("Increments the major version number in Cargo.toml")
                .usage("cargo-semver major")
                .action(major_action)
                .flag(pre_release_flag)
                .flag(config_file_flag.clone()),
        )
        .command(
            Command::new("pre")
                .description("Increments the pre-release version number in Cargo.toml")
                .usage("cargo-semver pre [alpha|beta]")
                .action(pre_action)
                .flag(config_file_flag),
        );

    app.run(args);
}

/// The default `cargo-semver` commands returns the
/// current version set in `Config.toml`.
fn default_action(c: &Context) {
    let version = Version::new(c);

    println!("{}", version.version);
}

/// Set the version in `Config.toml` to the `VALUE` input
/// in `cargo-semver set [VERSION]`.
fn set_version_action(c: &Context) {
    let mut version = Version::new(c);

    // the new version argument
    let version_arg = c.args.join(" ");

    let new_version = match version_arg.as_str() {
        v => match semver::Version::parse(&v) {
            Ok(v) => v,
            Err(err) => panic!("{}", err),
        },
    };

    version.set(new_version);
}

fn bump_version(c: &Context, bump: Bump) {
    let mut version = Version::new(c);

    let pre_flag = match c.string_flag("pre") {
        Ok(flag) => Some(flag),
        _ => None,
    };

    version.bump(bump, pre_flag)
}

/// Increments the patch version number.
fn patch_action(c: &Context) {
    bump_version(c, Bump::Patch)
}

/// Increments the minor version number.
fn minor_action(c: &Context) {
    bump_version(c, Bump::Minor)
}

/// Increments the major version number.
fn major_action(c: &Context) {
    bump_version(c, Bump::Major)
}

/// Increments the pre-release version number.
fn pre_action(c: &Context) {
    let value = c.args.join(" ");
    bump_version(c, Bump::Pre(value));
}
