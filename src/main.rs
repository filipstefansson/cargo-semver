mod config;
mod version;

use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;
use version::{Bump, Version};

#[cfg(not(tarpaulin_include))]
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
                .action(set_action)
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

fn set_version(c: &Context, version_arg: &str) -> String {
    let mut version = Version::new(c);

    let new_version = match version_arg {
        v => match semver::Version::parse(&v) {
            Ok(v) => v,
            Err(err) => panic!("{}", err),
        },
    };

    let new_version = version.set(new_version);
    println!("{}", new_version);
    new_version
}

fn bump_version(c: &Context, bump: Bump) -> String {
    let mut version = Version::new(c);

    let pre_flag = match c.string_flag("pre") {
        Ok(flag) => Some(flag),
        _ => None,
    };

    let new_version = version.bump(bump, pre_flag);
    println!("{}", new_version);
    new_version
}

/// Set the version in `Config.toml` to the `VALUE` input
/// in `cargo-semver set [VERSION]`.
fn set_action(c: &Context) {
    // the new version argument
    let version_arg = c.args.join(" ");

    set_version(c, &version_arg);
}

/// Increments the patch version number.
fn patch_action(c: &Context) {
    bump_version(c, Bump::Patch);
}

/// Increments the minor version number.
fn minor_action(c: &Context) {
    bump_version(c, Bump::Minor);
}

/// Increments the major version number.
fn major_action(c: &Context) {
    bump_version(c, Bump::Major);
}

/// Increments the pre-release version number.
fn pre_action(c: &Context) {
    let value = c.args.join(" ");
    bump_version(c, Bump::Pre(value));
}

#[cfg(test)]
mod tests {
    use super::*;
    use predicates::prelude::*;
    use seahorse::{Context, Flag, FlagType};
    use std::{fs, io::Write};
    use tempfile::NamedTempFile;

    fn setup_test_context(
        file: &mut NamedTempFile,
        version: &str,
        extra_args: &mut Vec<String>,
    ) -> Context {
        let file_has_content = file.as_file().metadata().unwrap().len() > 0;
        if !file_has_content {
            writeln!(
                file,
                "[package]\nversion = \"{}\"\n\n[dependencies]\nversion = \"{}\"",
                version, version,
            )
            .unwrap();
        }

        let config_path = file.path().to_str().unwrap().to_string();
        let config_flag = Flag::new("config", FlagType::String);
        let pre_flag = Flag::new("pre", FlagType::String);

        let mut args = vec!["--config".to_string(), config_path];
        args.append(extra_args);

        Context::new(args, Some(vec![config_flag, pre_flag]), "".to_string())
    }

    fn get_file_path(file: &mut NamedTempFile) -> String {
        file.path().to_str().unwrap().to_string()
    }

    #[test]
    fn test_bump_version() {
        let mut file = NamedTempFile::new().unwrap();
        let context = setup_test_context(&mut file, "1.0.0", &mut vec![]);
        let path = get_file_path(&mut file);

        major_action(&context);
        let contains = predicate::str::contains("2.0.0");
        assert_eq!(true, contains.eval(&fs::read_to_string(&path).unwrap()));

        minor_action(&context);
        let contains = predicate::str::contains("2.1.0");
        assert_eq!(true, contains.eval(&fs::read_to_string(&path).unwrap()));

        patch_action(&context);
        let contains = predicate::str::contains("2.1.1");
        assert_eq!(true, contains.eval(&fs::read_to_string(&path).unwrap()));

        let new_context = setup_test_context(&mut file, "0", &mut vec!["alpha".to_string()]);
        pre_action(&new_context);
        let contains = predicate::str::contains("2.1.1-alpha.1");
        assert_eq!(true, contains.eval(&fs::read_to_string(&path).unwrap()));

        pre_action(&new_context);
        let contains = predicate::str::contains("2.1.1-alpha.2");
        assert_eq!(true, contains.eval(&fs::read_to_string(&path).unwrap()));

        let new_context = setup_test_context(&mut file, "0", &mut vec!["beta".to_string()]);
        pre_action(&new_context);
        let contains = predicate::str::contains("2.1.1-beta.1");
        assert_eq!(true, contains.eval(&fs::read_to_string(&path).unwrap()));

        let new_context = setup_test_context(
            &mut file,
            "0",
            &mut vec!["--pre".to_string(), "beta".to_string()],
        );
        major_action(&new_context);
        let contains = predicate::str::contains("3.0.0-beta.1");
        assert_eq!(true, contains.eval(&fs::read_to_string(&path).unwrap()));

        major_action(&context);
        let contains = predicate::str::contains("4.0.0");
        assert_eq!(true, contains.eval(&fs::read_to_string(&path).unwrap()));

        let new_context = setup_test_context(&mut file, "0", &mut vec!["1.5.0".to_string()]);
        set_action(&new_context);
        let contains = predicate::str::contains("1.5.0");
        assert_eq!(true, contains.eval(&fs::read_to_string(&path).unwrap()));
    }
}
