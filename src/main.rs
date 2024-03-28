use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::env;

#[derive(Debug, Default)]
struct Configuration {
    ip: String,
    port: u32,
    address: String,
}

impl Configuration {
    fn new() -> Self {
        Self {
            ip: "0.0.0.0".to_string(),
            port: 7070,
            address: format!("{}:{}", "0.0.0.0", 7070),
        }
    }

    fn edit_configuration(&mut self, new_ip: String, new_port: u32) {
        self.ip = new_ip.clone();
        self.port = new_port.clone();
        self.address = format!("{}:{}", new_ip, new_port);
    }
}

#[tokio::main]
async fn main() {
    let mut server_conf = Configuration::new();
    server_conf.edit_configuration("10.0.0.73".to_string(), 7070);
    println!("INITIALIZING SERVER WITH CONFIGURATION {}:{}", server_conf.ip, server_conf.port);

    // Set the current working directory to the directory containing the file
    let current_dir = env::current_dir().expect("Failed to get current directory");

    println!("CURRENT WORKING DIRECTORY: {}", current_dir.to_string_lossy());

    let tcp_listener = TcpListener::bind(&server_conf.address).unwrap();
    println!("SERVER INITIALIZED, TCP LISTENER CONFIGURED");
    println!("LISTENING ON PORT {} AT IP ADDRESS {}", server_conf.port, server_conf.ip);

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
            }
        }
    }
}

async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0; 512];
    stream.read(&mut buf)?;

    let client_request = String::from_utf8_lossy(&buf[..]);
    println!("REQUEST RECEIVED FROM CLIENT! CONTENTS: {}", client_request);

    let response = request_handler(client_request.to_string());

    stream.write_all(response.as_bytes())?;

    Ok(())
}

fn request_handler(request: String) -> String {
    let response = match request.trim() {
        _ => fs::read_to_string("./src/gopher-hole/gophermap")
    };
    
    response.unwrap()

}
