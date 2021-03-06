use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, middleware::Logger};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::{fs, result};
use env_logger::Env;
use std::net::{ToSocketAddrs, SocketAddr, IpAddr};


#[derive(Debug, Clone)]
pub enum InstanceError {
    NotImplemented,
    FileConflict,
    InvalidBindAddr,
    ServerBindError,
}

#[derive(Debug, Clone)]
pub enum InstanceType {
    Proxy(String),
    Static(String),
    //RandomicBalancer(Vec<>)
}

#[derive(Debug)]
pub struct Instance {
    bind_addr: SocketAddr,
    instance_type: InstanceType,
    server: actix_web::dev::Server,
    status: String
}

impl Instance {
    pub fn create(
        bind_addr: String,
        bind_port: u16,
        instance_type: InstanceType,
    ) -> Result<Self, InstanceError> {

        let mut instance_dirname = "";

        match instance_type.clone() {
            // When Instance type is static file server
            // Create file if not exists
            InstanceType::Static(x) => {

            },
            _ => return Err(InstanceError::NotImplemented)
        }

        let validate_bind_addr = bind_addr.parse::<Ipv4Addr>();
        let mut bind_addr: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

        match validate_bind_addr {
            Err(_) => return Err(InstanceError::InvalidBindAddr),
            Ok(parsed_ip) => bind_addr = parsed_ip,
        }

        // Add port to parsed ipv4 and transform into SocketAddr
        let bind_addr = SocketAddr::from((IpAddr::V4(bind_addr), bind_port));

        // Only static files
        let server_builder =
            HttpServer::new(|| App::new()
                .service(actix_files::Files::new("/", "instances/default/public").index_file("index.html"))
            ).workers(1)
                .bind(bind_addr.clone());

        match server_builder {
            Err(_) => return Err(InstanceError::ServerBindError),
            Ok(server) => {
                return Ok(Self {
                    bind_addr,
                    instance_type,
                    server: server.run(),
                    status: "Running".to_string()
                })
            }
        }
    }

    /// Stop instance server
    pub async fn stop(&mut self) -> Result<(), ()> {
        self.server.stop(true).await;
        Ok(())
    }

    pub fn get_info(&self) -> HashMap<&'static str, String> {
        let mut info: HashMap<&'static str, String> = HashMap::new();

        info.insert("status", self.status.clone());
        info.insert("bind_addr", self.bind_addr.to_string());
        info.insert("type", String::from("Not implemented"));

        info
    }
}
