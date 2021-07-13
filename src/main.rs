use std::path::Path;
use env_logger::Env;

mod instance;
use instance::{Instance, InstanceType};

#[actix_web::main]
async fn main() {

    env_logger::Builder::from_env(
        Env::default()
            .default_filter_or("info")
            .default_filter_or("error")
            .default_filter_or("warn")
            .default_filter_or("debug")
        ).init();


    let default_path = Path::new("instances/default");

    if default_path.exists() {
        println!("A pasta existe!")
    }

    let default_path = default_path.to_path_buf();

    for i in default_path.read_dir().unwrap() {
        println!("{:?}", i);
    }

    let example = Instance::new("127.0.0.1", 3000, InstanceType::Static(default_path));

    
    println!("{:#?}", example);

    loop {

    }

}
