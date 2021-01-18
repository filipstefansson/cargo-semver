use std::fs;

pub struct Config;

impl Config {
    /// Loads the `Config.toml` file into a string.
    pub fn read_config(path: &str) -> String {
        let data = fs::read_to_string(path).expect(&format!("unable to read {}", path));
        let config = data.parse::<String>().unwrap();

        return config;
    }

    /// Overwrites `Config.toml` file with the `config` arg.
    pub fn write_config(path: &str, config: &String) {
        fs::write(path, config.to_string()).expect(&format!("unable to write to {}", path));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use predicates::prelude::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_config() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        let path = file.path().to_str().unwrap();

        let config = Config::read_config(path);

        let contains = predicate::str::contains("test content");
        assert_eq!(true, contains.eval(&config));
    }

    #[test]
    fn test_write_config() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        let path = file.path().to_str().unwrap();

        Config::write_config(path, &"new content".to_string());

        let contains = predicate::str::contains("new content");
        assert_eq!(true, contains.eval(&fs::read_to_string(path).unwrap()));
    }
}
