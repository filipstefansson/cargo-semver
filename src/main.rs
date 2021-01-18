mod config;
mod version;

use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use std::env;
use version::{Bump, Version};

#[cfg(not(tarpaulin_include))]
fn main() {
    // create a sub command for cargo
    let matches = App::new("cargo-semver")
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("semver")
                .version(crate_version!())
                .bin_name("cargo")
                .about("Read or update the version in your Cargo.toml file")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("Use a custom config file")
                        .takes_value(true)
                        .global(true),
                )
                .subcommand(
                    SubCommand::with_name("set")
                        .about("Set a specific version")
                        .usage("cargo-semver set [VERSION]")
                        .arg(
                            Arg::with_name("VERSION")
                                .help("A semantic version")
                                .required(true)
                                .index(1),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("get")
                        .about("Prints the current version")
                        .usage("cargo-semver get"),
                )
                .subcommand(
                    SubCommand::with_name("bump")
                        .about("Increments the version number")
                        .usage("cargo-semver bump [TYPE] [PRE-RELEASE]")
                        .arg(
                            Arg::with_name("TYPE")
                                .required(true)
                                .index(1)
                                .possible_values(&["major", "minor", "patch", "pre"])
                                .help("Increment type"),
                        )
                        .arg(
                            Arg::with_name("PRE-RELEASE")
                                .required(false)
                                .index(2)
                                .help("Add a pre-release version (optional)"),
                        ),
                ),
        )
        .get_matches();

    // go down one sub command level from `cargo semver` to just `semver`
    let matches = matches.subcommand_matches("semver").unwrap();

    // create a version from the config file
    let config_path = matches.value_of("config").unwrap_or("Cargo.toml");
    let mut version = Version::new(config_path);

    // $ cargo semver get
    if matches.subcommand_matches("get").is_some() {
        println!("{}", version.version);
        return;
    }

    // $ cargo semver set x.x.x
    if let Some(matches) = matches.subcommand_matches("set") {
        let value = matches.value_of("VERSION").unwrap();
        let new_version = version.set(value);
        println!("{}", new_version);

        return;
    }

    // $ cargo semver bump major|minor|patch|pre
    if let Some(matches) = matches.subcommand_matches("bump") {
        let mut pre_arg = match matches.value_of("PRE-RELEASE") {
            Some(arg) => Some(arg),
            None => None,
        };

        let bump = match matches.value_of("TYPE") {
            Some("major") => Bump::Major,
            Some("minor") => Bump::Minor,
            Some("patch") => Bump::Patch,
            Some("pre") => {
                // use pre in bump and then set it to none
                // so we don't get the pre-release logic for the other bumps
                let pre = pre_arg.unwrap_or("").to_string();
                pre_arg = None;
                Bump::Pre(pre)
            }
            _ => panic!("invalid [TYPE] argument"),
        };

        let new_version = version.bump(bump, pre_arg);
        println!("{}", new_version);

        return;
    }
}
