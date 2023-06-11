// configuration file functions
// 20230611

use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub data_dir: String,
    pub db_filename: String,
}

pub fn read_config_file() -> Config {
    let dot_config_path = get_dot_config_path();
    let config_file_path = dot_config_path.join("config.json");

    // open configuration file
    let config_file = match File::open(&config_file_path) {
        Err(_msg) => {
            create_config_file();
            process::exit(1);
        }
        Ok(file) => file,
    };

    // read configuration values from config.json
    let config: Config = serde_json::from_reader(config_file)
        .expect("Could not read values from configuration file");

    config
}

fn get_dot_config_path() -> PathBuf {
    let prog_path = env::current_exe().unwrap();
    let prog_file_name = prog_path.file_name().unwrap();
    let user_home_dir_path = dirs::home_dir().unwrap();

    // create configuration folder if it doesn't exist (/home/user/.config/program_name) if it doesn't exist
    let dot_config_path = user_home_dir_path.join(".config").join(prog_file_name);
    fs::create_dir_all(&dot_config_path).expect("cannot create config folder");

    dot_config_path
}

fn create_config_file() {
    let dot_config_path = get_dot_config_path();
    let config_file_path = dot_config_path.join("config.json");
    let data_dir_path = &dot_config_path;

    // setup default values for config file
    let config = Config {
        data_dir: String::from(data_dir_path.to_string_lossy()),
        db_filename: String::from("beer.db"),
    };

    // create config file
    let cfg_file = File::create(&config_file_path).expect("config file not created");
    serde_json::to_writer_pretty(cfg_file, &config).expect("error writing to config");

    println!("Config file not found...");
    println!("Config file created: {:?}", &config_file_path);
    println!("Please start program again to use new configuration file.");
}
