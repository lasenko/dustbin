use std::{env, fs};
use std::path::PathBuf;
use auto_launch;
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    package: Package,
}

/*pub fn reset() {

}*/

pub fn autorun() {
    let cfg_string = fs::read_to_string("Cargo.toml").expect("Failed to parse Cargo.toml into string");
    let config: Config = toml::from_str(&cfg_string).expect("Failed to parse cfg_string into config");
    let app_name= config.package.name;

    let app_path = grab_path(); 

    let auto_launch = auto_launch::AutoLaunch::new(&app_name, &app_path, &[] as &[&str]);

    // Enable auto-launch
    if auto_launch.is_enabled().unwrap() {
        auto_launch.disable().expect("Error disabling auto-launch")
    } else{
        match auto_launch.enable() {
            Ok(_) => println!("Auto-launch enabled successfully!"),
            Err(e) => eprintln!("Error enabling auto-launch: {}", e),
        }
    }
}

fn grab_path() -> String {
    let path = env::current_exe().expect("Failed to determine path to the exe");
    // converting to String
    path.into_os_string().into_string().unwrap()
}