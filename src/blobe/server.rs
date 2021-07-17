use super::{Instance, InstanceType};
use cli_table::CellStruct;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use git2::Repository;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Server {
    blobes: HashMap<String, Instance>,
}

impl Server {
    /// User this to create a new BlobeServer
    pub fn new() -> Self {
        Self {
            blobes: HashMap::new(),
        }
    }

    /// Use this to execute commands on server
    /// Command pattern: module cmd arg1 agr2...
    pub fn command(&mut self, s: String) {
        if s.is_empty() {
            return;
        };

        let command = s.split(" ").collect::<Vec<&str>>();

        // Get the module of command
        let mut module = "";
        match command.get(0) {
            Some(value) => module = *value,
            None => {
                error!(target: "Server::command()", "It's probably a bug. command module not found.")
            }
        }

        match module {
            "instance" => {
                // Get cmd of module
                let mut cmd = "";
                match command.get(1) {
                    Some(value) => cmd = *value,
                    None => {
                        error!(target: "Server", "Type '{} help' to see module commands", module)
                    }
                }

                match cmd {
                    "new" => {
                        warn!(target: "Server", "Sorry, but this command has not yet been implemented.")
                    }
                    // Load try loading instance from instances folder, using name of instance folder;
                    "load" => {
                        let mut instance_folder_name = "";
                        match command.get(2) {
                            Some(value) => instance_folder_name = *value,
                            None => {
                                error!(target: "Loader", "Bad request. you dont send instance name: 'instance load ???'. Try again... example: instance load my-website");
                                return;
                            }
                        }

                        let instances_folder = Path::new("instances");
                        let instances_folder: PathBuf = instances_folder.join(instance_folder_name);

                        info!(target: "Server", "Try loading {}", instance_folder_name);

                        if instances_folder.exists() {
                            info!(target: "Server", "Aquivo encontrado.");
                            self.load_instance(instances_folder);
                        } else {
                            error!(target: "Server", "the {} instance folder does not exist. Try 'instance new {}', to generate a new instance.", instance_folder_name, instance_folder_name);
                        }
                    }
                    "stop" => {
                        let mut instance_name = "";
                        match command.get(2) {
                            Some(value) => instance_name = *value,
                            None => {
                                error!(target: "Loader", "Bad request. you dont send instance name: 'instance stop ???'. Try again... example: instance load my-website");
                                return;
                            }
                        }
                        self.stop_instance(instance_name.to_string());
                    }
                    "unload" => (),
                    "pause" => {
                        warn!(target: "Server", "Sorry, but this command has not yet been implemented.")
                    }
                    "resume" => {
                        warn!(target: "Server", "Sorry, but this command has not yet been implemented.")
                    }
                    "list" => {
                        let mut table: Vec<Vec<CellStruct>>= Vec::new();

                        for (name, instance) in self.blobes.iter() {

                            let info = instance.get_info();

                            let mut status = String::new();
                            if let Some(data) = info.get("status") {
                                status = data.clone();
                            }

                            let mut bind_addr = String::new();
                            if let Some(data) = info.get("bind_addr") {
                                bind_addr = data.clone();
                            }

                            table.push(vec![name.cell(), bind_addr.cell()]);
                        }


                        let table = table.table().title(vec![
                            "Name".cell().bold(true),
                            "Bind Addr".cell().bold(true),
                        ]);
                        
                        print_stdout(table);
                    }
                    "status" => {
                        warn!(target: "Server", "Sorry, but this command has not yet been implemented.")
                    }
                    "clone" => {
                        warn!(target: "Server", "Sorry, but this command has not yet been implemented.")
                    }
                    _ => {
                        error!(target: "Server", "Command not found. Type '{} help' to see module commands.", module)
                    }
                }
            }
            _ => error!(target: "Server", "Module not exists. Type 'help' for see all modules."),
        }
    }

    // Use this to load and start instance
    fn load_instance(&mut self, path: PathBuf) {
        info!(target: "Server", "try loading: {:?}", path.to_str().unwrap());

        let mut config = config::Config::default();
        if let Err(_) = config.merge(config::File::from(path.join("config.toml"))) {
            return error!("Cant load config file from {}", path.to_str().unwrap());
        };

        match config.try_into::<HashMap<String, String>>() {
            Ok(map) => {
                info!(target: "Server", "Config file loaded");

                // Try get instance name
                let mut instance_name = String::new();
                match map.get("name") {
                    Some(value) => instance_name = value.clone(),
                    None => {
                        error!(target: "Nameless Instance", "cant find 'name' on instance config file");
                        return;
                    }
                }

                //Try get instance bind_addr
                let mut bind_addr = String::new();
                match map.get("bind_addr") {
                    Some(value) => bind_addr = value.clone(),
                    None => {
                        error!(target: instance_name.clone().as_str(), "cant find bind_port on instance config file");
                        return;
                    }
                }

                let mut bind_port: u16 = 1234;
                match map.get("bind_port") {
                    Some(value) => bind_port = value.clone().parse::<u16>().unwrap(),
                    None => {
                        error!(target: instance_name.clone().as_str(), "cant find bind_port on instance config file");
                        return;
                    }
                }

                println!("Isso é apenas um item da configuracao: {}", instance_name);

                self.blobes.insert(
                    instance_name.clone(),
                    Instance::create(bind_addr, bind_port, InstanceType::Static(instance_name))
                        .unwrap(),
                );
            }
            Err(e) => error!(target: "Server", "Broken config file: {}" ,e),
        }
    }

    /// Unload all instances
    pub async fn unload_all(&mut self) {
        info!(target: "Server", "Unloading all blobe instances...");

        for (name, instance) in self.blobes.iter_mut() {
            match instance.stop().await {
                Ok(_) => info!(target: "Server", "Unloaded {}", name),
                Err(_) => warn!(target: "Server", "Cant unload {}, this can cause a problem", name),
            }
        }
        info!(target: "Server", "All instances unloaded.");
    }

    /// Unload specify instance
    pub async fn unload_instance(&mut self, instance_name: String) {
        info!(target: "Server", "Try unload {}.", instance_name.as_str());
        match self.blobes.get_mut(instance_name.as_str()) {
            Some(instance) => {
                instance.stop();
                self.blobes.remove(instance_name.as_str());
                info!(target: "Server", "{} instance has been unloaded!", instance_name.as_str());
            }
            None => {
                error!(target: "Server", "Cant find '{}'", instance_name.as_str());
            }
        }
    }

    /// stop http server instance
    pub async fn stop_instance(&mut self, instance_name: String) {
        info!(target: "Server", "Try unload {}.", instance_name.as_str());
        match self.blobes.get_mut(instance_name.as_str()) {
            Some(instance) => {
                instance.stop();
                info!(target: "Server", "{} instance has been unloaded!", instance_name.as_str());
            }
            None => {
                error!(target: "Server", "Cant find '{}'", instance_name.as_str());
            }
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
                self.load_instance(instance_path);
            }
        }
    }
}
