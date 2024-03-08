use std::borrow::Cow;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{clone, fs};

use futures::{FutureExt, StreamExt};

#[derive(Debug, Default, Clone)]
struct Configuration {
    ip: String,
    port: u32,
    address: String
}

impl Configuration {
    fn new() -> Self {
        Self {
            ip: "0.0.0.0".to_string(),
            port: 7070,
            address: format!("{}:{}", "0.0.0.0", 7070)
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

    let tcp_listener = TcpListener::bind(server_conf.address).unwrap();
    println!("SERVER INITIALIZED, TCP LISTENER CONFIGURED");
    println!("LISTENING ON PORT {} AT IP ADDRESS {}", server_conf.ip, server_conf.port);

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream).await;
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
            }
        }
    }

}

async fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();

    let client_request = String::from_utf8_lossy(&buf[..]);
    println!("REQUEST RECEIVED FROM CLIENT! CONTENTS: {}", client_request);

    let response = request_handler(client_request.to_string());

    stream.write_all(response.as_bytes()).unwrap();
}

fn request_handler(request: String) -> String {

    let response = match request.trim() {
        _ => throw_gopher("/home/fizbin/lair/projects/rust/mole-node/src/gophermap"),
    };

    return response
    
}

fn throw_gopher(file_path: &str) -> String { // lob a live gopher with a makeshift catapult
    let content = fs::read_to_string(file_path).unwrap();
    return content
}

fn throw_error(message: &str) -> String {
    let error = format!("ERROR: {}", message);
    error
}