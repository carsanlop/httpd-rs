use std::error::Error;
use std::fs;
use std::net;
use std::path;
use std::io::prelude::*;
use crate::pool;
use crate::http;

type Operation = Result<(), Box<dyn Error>>;

pub struct Config {
    pub port: u16,
    pub source: String,
    pub buffer_size: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            port: 8080,
            source: String::from("wwwroot"),
            buffer_size: 1024,
        }
    }
}

pub struct Server {
    listener: net::TcpListener,
    pool: pool::ThreadPool,
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Server {
        let addr = net::SocketAddr::from(([127, 0, 0, 1], config.port));
        let listener = net::TcpListener::bind(addr).unwrap();
        let pool = pool::ThreadPool::new(12);

        Server {
            listener,
            pool,
            config,
        }
    }

    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let root = self.config.source.clone();

            self.pool.execute(move || {
                match handle(stream, &root) {
                    Ok(_) => (),
                    Err(error) => eprintln!("An error ocurred processing a request: {}", error)
        
                }
            });
        }
    }
}

fn handle(mut stream: net::TcpStream, root: &str) -> Operation {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Try to parse the request.
    let request = match http::Request::new(&buffer) {
        Some(v) => v,
        None => return Err("Could not parse the request".into())
    };

    // Try to load and send the requested resource.
    match load(root, request.resource) {
        Some(content) => {
            send(&mut stream, http::Response::new(200, "OK", &content));
        },
        None => {
            send(&mut stream, http::Response::new(404, "Not Found", ""));
        }
    }

    Ok(())
}

fn load(root: &str, resource: &str) -> Option<String> {
    let mut path = path::PathBuf::from(root);

    // Determine the source to load
    match resource {
        "/" => path.push("index.html"),
        _ => path.push(resource.strip_prefix("/").unwrap_or_default())
    }

    // Try to load the actual contents for the file.
    match fs::read_to_string(&path) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn send(stream: &mut net::TcpStream, response: http::Response) {
    let s = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
        response.status_code,
        response.status_reason,
        response.content.len(),
        response.content
    );

    stream.write(s.as_bytes()).unwrap();
    stream.flush().unwrap();

}

