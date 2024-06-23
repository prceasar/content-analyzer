use clap::Parser;
use inquire::Password;
use serde::{Serialize, Deserialize};
use serde_json;
use directories::ProjectDirs;
use std::fs::{self, File};
use std::io::{ Write, Read}; // Added Read here

#[derive(Parser, Debug)]
#[clap(author = "Jaime Morales <jaime.raul.morales@gmail.com>", version = "0.1.0", about = "Analyzes your contentful space to give you insights about the relationships in the data", long_about = None)]
struct Cli {
    /// The ID of the space
    #[clap(index = 1)]
    space_id: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    space_configs: HashMap<String,SpaceConfig>,
}

#[derive(Serialize, Deserialize)]
struct SpaceConfig {
    api_token: String,
}

fn main() {
    let cli = Cli::parse();

    let project_dirs = ProjectDirs::from("com", "jrm", "content-analyzer").unwrap();
    let config_dir = project_dirs.config_dir();
    fs::create_dir_all(config_dir).unwrap();
    let config_path = config_dir.join("config.json");
    let mut config_file = File::open(&config_path).unwrap();
    let mut config_data = String::new();
    config_file.read_to_string(&mut config_data).unwrap();
    let config: Config = serde_json::from_str(&config_data).unwrap();

    let api_token = if config.api_token.is_empty() {
        prompt_for_token(&config_path, &cli.space_id)
    } else {
        let use_existing_token = inquire::Confirm::new("Use existing API token?")
            .with_default(true)
            .prompt()
            .unwrap_or(false);
        if use_existing_token {
            config.api_token
        } else {
            prompt_for_token(&config_path, &cli.space_id)  
        }
    };

    // write the token back to the config and save it
    // based on the passed in space_id, it should get the right config and override that with the new token
    let space_config = config.space_configs.get(&cli.space_id).unwrap();
    space_config.api_token = api_token.clone();
    config.space_configs.insert(cli.space_id, space_config);
    let config = Config { space_configs: config.space_configs };
    let config_data = serde_json::to_string(&config).unwrap();
    let mut config_file = File::create(config_path).unwrap();
    config_file.write_all(config_data.as_bytes()).unwrap();

    println!("Using space ID: {}, {}", cli.space_id, api_token);
    // Use the api_token as needed here
}

fn prompt_for_token(config_path: &std::path::Path, space_id: &String) -> String {
    let token = Password::new("Please enter your API token:")
        .with_display_toggle_enabled()
        .without_confirmation()
        .prompt()
        .unwrap();
    let mut config: Config = serde_json::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();
    let space_config = config.space_configs.entry(space_id.clone()).or_insert(SpaceConfig { api_token: token.clone() });
    space_config.api_token = token.clone();
    let config_data = serde_json::to_string(&config).unwrap();
    let mut config_file = File::create(config_path).unwrap();
    config_file.write_all(config_data.as_bytes()).unwrap();
    token
}