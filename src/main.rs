
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod blobe;
use blobe::{Server};

#[actix_web::main]
async fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("info")
            .default_filter_or("error")
            .default_filter_or("warn")
            //.default_filter_or("debug")
            //.default_filter_or("trace")
        ).init();
    
    let mut server = Server::new();
    server.init().await;

    let mut reader = Editor::<()>::new();

    loop {
        let readline = reader.readline(">> ");
        match readline {
            Ok(line) => {
                reader.add_history_entry(line.as_str());
                server.command(line.clone());
            }
            Err(ReadlineError::Interrupted) => {
                server.unload_all();
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
}