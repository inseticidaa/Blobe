use super::{Instance, InstanceType};
use git2::Repository;
use log::{debug, error, info, log_enabled, warn, Level};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Server {
    blobes: HashMap<&'static str, Instance>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            blobes: HashMap::new(),
        }
    }

    pub fn command(&self, s: String) {
        let commands = s.split(" ").collect::<Vec<_>>();

        match commands[0] {
            "instance" => println!("instance called"),
            _ => println!("command not found"),
        }
    }

    // Use this to load and start instance
    async fn load_instance(&mut self, path: PathBuf) {
        info!(target: "Server", "try loading: {:?}", path.to_str().unwrap());

        let mut config = config::Config::default();
        if let Err(_) = config.merge(config::File::from(path.join("config.toml"))) {
            return error!("Cant load config file from {}", path.to_str().unwrap());
        };

        match config.try_into::<HashMap<String, String>>() {
            Ok(map) => {
                info!(target: "Server", "Config file loaded");

                // Try get instance name
                let mut instance_name = "";
                match map.get("name") {
                    Some(value) => instance_name = value,
                    None => {
                        error!(target: "Nameless Instance", "cant find 'name' on instance config file");
                        return;
                    }                 
                }

                // Try get instance bind_addr
                let mut bind_addr = "";
                match map.get("bind_addr") {
                    Some(value) => bind_addr = value,
                    None => {
                        error!(target: instance_name.clone(), "cant find bind_port on instance config file");
                        return;
                    }                 
                }

                println!("Isso é apenas um item da configuracao: {}", bind_addr);

                self.blobes.insert(
                    "default",
                    Instance::create(bind_addr, 8080, InstanceType::Static("default")).unwrap(),
                );
            }
            Err(_) => error!(target: "Server", "Broken config file"),
        }
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
                }
            };

            // Donwload default first instance
            info!(target: "Server", "Downloading default instance model...");
            match Repository::clone(
                "https://github.com/inseticidaa/blobe-default-instance.git",
                instances_folder.join("default"),
            ) {
                Ok(_) => info!(target: "Server", "Downloading complete"),
                Err(_) => warn!(target: "Server", "Error on downloading default instance model"),
            }
        }

        // Try read instances dir
        let instances = instances_folder
            .read_dir()
            .expect("Cant read instances folder on start server struct");
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
