#[macro_use]
extern crate log;

use std::{fs, path::Path};

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod blobe;
use blobe::{Server};

#[actix_web::main]
async fn main() {

    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

    let mut server = Server::new();
    server.init().await;

    let mut reader = Editor::<()>::new();

    if reader.load_history("temp/history").is_ok() {
        info!("Command history loaded");
    }

    loop {
        let readline = reader.readline(">> ");
        match readline {
            Ok(line) => {
                let line = line.to_lowercase();
                reader.add_history_entry(line.as_str());
                println!("{}", line);
                server.command(line.clone());
            }
            Err(ReadlineError::Interrupted) => {
                server.unload_all().await;
                info!(target: "For you", "Good Bye :D");
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    let temp = Path::new("temp");

    if !temp.exists() {
        fs::create_dir(temp);
    }

    if reader.save_history("temp/history").is_err() {
        error!("Cant save command hitory")
    };
}