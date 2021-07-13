use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, middleware::Logger};
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use env_logger::Env;
use std::net::{ToSocketAddrs, SocketAddr, IpAddr};


#[derive(Debug, Clone)]
pub enum InstanceError {
    NotImplemented,
    InvalidBindAddr,
    ServerBindError,
}

#[derive(Debug, Clone)]
pub enum InstanceType {
    Proxy(&'static str),
    Static(PathBuf),
    //RandomicBalancer(Vec<>)
}

#[derive(Debug)]
pub struct Instance {
    bind_addr: SocketAddr,
    instance_type: InstanceType,
    server: actix_web::dev::Server,
}

impl Instance {
    pub fn new(
        bind_addr: &'static str,
        bind_port: u16,
        instance_type: InstanceType,
    ) -> Result<Self, InstanceError> {

        match instance_type.clone() {
            // When Instance type is static file server
            InstanceType::Static(_) => (),
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
                .wrap(Logger::default())
                .wrap(Logger::new("Default Instance"))
            )
                .bind(bind_addr.clone());

        match server_builder {
            Err(_) => return Err(InstanceError::ServerBindError),
            Ok(server) => {
                return Ok(Self {
                    bind_addr,
                    instance_type,
                    server: server.run(),
                })
            }
        }
    }
}
