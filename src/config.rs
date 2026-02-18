use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(rename = "slots")]
    pub slots: Vec<SlotConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SlotConfig {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub slot_type: String,
}

fn config_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".config/darwin-slot/config.toml")
}

const DEFAULT_CONFIG: &str = r#"[[slots]]
name = "local-1"
path = "/Users/cyprien/Software/darwin-local"
type = "local"

[[slots]]
name = "local-2"
path = "/Users/cyprien/Software/darwin-local-2"
type = "local"

[[slots]]
name = "local-3"
path = "/Users/cyprien/Software/darwin-local-3"
type = "local"

[[slots]]
name = "proto-1"
path = "/Users/cyprien/Software/darwin-proto"
type = "proto"

[[slots]]
name = "proto-2"
path = "/Users/cyprien/Software/darwin-proto-2"
type = "proto"

[[slots]]
name = "proto-3"
path = "/Users/cyprien/Software/darwin-proto-3"
type = "proto"
"#;

pub fn load_config() -> Config {
    let path = config_path();

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create config directory");
        }
        fs::write(&path, DEFAULT_CONFIG).expect("Failed to write default config");
        eprintln!("Created default config at {}", path.display());
    }

    let content = fs::read_to_string(&path).expect("Failed to read config file");
    toml::from_str(&content).expect("Failed to parse config file")
}
