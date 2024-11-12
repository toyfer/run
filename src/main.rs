use serde::Deserialize;
use std::fs;
use std::collections::HashMap;
use std::process::Command;

#[derive(Deserialize)]
struct Config {
    env_vars_path: String,
}

#[derive(Deserialize)]
struct EnvVarList {
    alias: HashMap<String, String>,
}
fn load_config() -> Config {
    let config_data = fs::read_to_string("config.toml").expect("Unable to read config file");
    toml::from_str(&config_data).expect("Unable to parse config file")
}

fn load_env_vars(path: &str) -> EnvVarList {
    let env_vars_data = fs::read_to_string(path).expect("Unable to read env vars file");
    toml::from_str(&env_vars_data).expect("Unable to parse env vars file")
}

fn launch_application(command: &str, args: &[String], env_vars: &HashMap<String, String>) {
    let mut cmd = Command::new(command);
    cmd.args(args);
    for (key, value) in env_vars {
        cmd.env(key, value);
    }
    cmd.spawn().expect("Failed to launch application");
}

fn main() {
    let config = load_config();
    let env_vars_list = load_env_vars(&config.env_vars_path);
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <alias> [args...]", args[0]);
        return
    }
    let alias = &args[1];
    let app_args = &args[2..];
    if let Some(command) = env_vars_list.alias.get(alias) {
        launch_application(command, app_args, &env_vars_list.alias);
    } else {
        eprintln!("Alias not found: {}", alias)
    }
}
