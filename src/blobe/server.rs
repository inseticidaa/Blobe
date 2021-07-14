use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use git2::Repository;
use super::Instance;
use log::{debug, error, log_enabled, info, Level, warn};


pub struct Server {
    blobes: HashMap<&'static str, Instance>
}

impl Server {
    pub fn new() -> Self {
        Self {
            blobes: HashMap::new()
        }
    }

    pub fn command(&self, s: String) {

        let commands = s.split(" ").collect::<Vec<_>>();

        match commands[0] {
            "instance" => println!("instance called"),
            _ => println!("command not found")
        }
    }

    // Use this to load and start instance
    async fn load_instance(&mut self, path: PathBuf) {
        info!(target: "Server", "try loading: {:?}", path.to_str().unwrap());
        warn!("to testando");
    }

    // Use this to load all instances
    pub async fn init(&mut self) {

        info!(target: "Server", "Loading instances...");
        
        let instances_folder = Path::new("instances");

        // Create instances folder if not exists
        if !instances_folder.exists() {

            info!(target: "Server", "Creating instances folder...");
            match fs::create_dir(instances_folder) {
                Ok(_) => info!(target: "Server", "Instances folder has been created"),
                Err(_) => {
                    error!(target: "Server", "Error on create instances folder");
                    panic!("Error on create instances folder");
                },
            };


            // Donwload default first instance
            info!(target: "Server", "Downloading default instance model...");
            match Repository::clone("https://github.com/inseticidaa/blobe-default-instance.git", instances_folder.join("default")) {
                Ok(_) => info!(target: "Server", "Downloading complete"),
                Err(_) => warn!(target: "Server", "Error on downloading default instance model")
            }
        } 

        // Try read instances dir
        let instances = instances_folder.read_dir().expect("Cant read instances folder on start server struct");
        
        // Try loading instances
        for instance_path in instances {

            let instance_path = instance_path.expect("ok");
            let instance_path = instance_path.path();

            if instance_path.is_dir() {
                self.load_instance(instance_path).await;
            }
        }
    }
}