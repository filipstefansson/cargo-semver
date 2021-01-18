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
