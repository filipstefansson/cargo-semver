use crate::config::Config;
use regex::Regex;
use seahorse::Context;
use semver::Identifier;
use std::process;

#[derive(Clone)]
pub enum Bump {
    Major,
    Minor,
    Patch,
    Pre(String),
}

#[derive(Debug)]
pub struct Version {
    /// the full version string in Config.toml to replace
    /// ie. `version = "1.0.0"`. We need this in the search
    /// and replace method, because it could be badly formatted
    /// like `version= "1.0.0"`.
    pub line: String,
    /// the new version
    pub version: semver::Version,
    config_content: String,
    config_path: String,
}

impl Version {
    /// Grabs the `version` from the config string.
    pub fn new(c: &Context) -> Version {
        let config_path = match c.string_flag("config") {
            Ok(p) => p,
            _ => "Cargo.toml".to_string(),
        };

        let config_content = Config::read_config(&config_path);

        let search = Regex::new(r#"version\s?=\s?"(.*?)""#).unwrap();
        let hits = search.captures_iter(&config_content);

        for hit in hits {
            // nothing to do here becuase it doesn't have any value
            if hit.iter().count() < 2 {
                continue;
            }

            let line = hit[0].to_string();
            match semver::Version::parse(&hit[1].to_string()) {
                Ok(version) => {
                    return Version {
                        config_path,
                        config_content,
                        line,
                        version,
                    }
                }
                Err(err) => {
                    panic!("{}", err);
                }
            }
        }

        panic!("failed to find version in Config.toml")
    }

    pub fn set(&mut self, version: semver::Version) -> String {
        self.version = version;

        self.update_config_version()
    }

    pub fn bump(&mut self, bump: Bump, pre_flag: Option<String>) -> String {
        match bump {
            Bump::Major => &self.version.increment_major(),
            Bump::Minor => &self.version.increment_minor(),
            Bump::Patch => &self.version.increment_patch(),
            Bump::Pre(pre) => &self.increment_pre(pre),
        };

        if let Some(pre_flag) = pre_flag {
            self.version.pre = vec![Identifier::AlphaNumeric(pre_flag), Identifier::Numeric(1)];
        }

        self.update_config_version()
    }

    fn update_config_version(&self) -> String {
        // validate version
        let string_version = &self.version.to_string();
        if let Err(err) = semver::Version::parse(string_version) {
            panic!("{}", err);
        }

        let new_version_line = format!("version = \"{}\"", &self.version.to_string());
        let new_config = &self
            .config_content
            .replacen(&self.line, &new_version_line, 1);

        // write config
        Config::write_config(&self.config_path, new_config);

        // run cargo check to update Cargo.lock
        process::Command::new("cargo")
            .arg("check")
            .output()
            .expect("failed to run `cargo check` to update Cargo.lock");

        self.version.to_string()
    }

    fn increment_pre(&mut self, pre_version: String) {
        if !self.version.is_prerelease() && pre_version.is_empty() {
            panic!("run `cargo-semver pre [alpha|beta]` first to add a new pre-release version.");
        }

        let is_same_pre = match &self.version.pre.first() {
            Some(Identifier::AlphaNumeric(v)) => v.to_string() == pre_version,
            _ => false,
        };

        // if the pre_version is the same as the current pre-release version, or if it's empty,
        // we should bump it from *.*.*-[pre_version].N to *.*.*-[pre_version].N+1
        // if not, then we set it to *-[pre_version].1
        if self.version.is_prerelease() && (is_same_pre || pre_version.is_empty()) {
            match &self.version.pre.last().unwrap() {
                Identifier::Numeric(v) => {
                    let mut pre = self.version.pre.clone();
                    pre.pop();
                    pre.push(Identifier::Numeric(v + 1));
                    self.version.pre = pre;
                }
                _ => panic!("could not increase pre-release number."),
            }
        } else {
            self.version.pre = vec![
                Identifier::AlphaNumeric(pre_version),
                Identifier::Numeric(1),
            ];
        }
    }
}
