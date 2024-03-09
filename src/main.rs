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

enum GopherType {
    TextFile, //0
    Directory, //1
    NameServer, //2
    Error, //3
    MacHQFiller, //4
    MSBin, //5, windows exe file
    UnixUnencoded, // 6 unencoded unix file (???)
    SearchServer, //7, dfk
    TelnetSesh, //8
    Bin, //9
    Gif, //g
}

impl GopherType {
    fn to_char(&self) -> char {
        match *self {
            GopherType::TextFile => '0',
            GopherType::Directory => '1',
            GopherType::NameServer => '2',
            GopherType::Error => '3',
            GopherType::MacHQFiller => '4', // i have no fucking clue what this means
            GopherType::MSBin => '5', // enjoy having ur data sold pleb
            GopherType::UnixUnencoded => '6', // dont know what these are either
            GopherType::SearchServer => '7',
            GopherType::TelnetSesh => '8',
            GopherType::Bin => '9',
            GopherType::Gif => 'g',
        }
    }
}

#[tokio::main]
async fn main() {
    let mut server_conf = Configuration::new();
    server_conf.edit_configuration("10.11.115.209".to_string(), 7070);
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
        "" => throw_gopher("./gophermap"), // Assuming 'gophermap' is the file name
        _ => throw_gopher("")
    };

    response
}

fn throw_gopher(file_name: &str) -> String {
    match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(e) => throw_error(&format!("Failed to read file: {}", e)),
    }
}

fn throw_error(message: &str) -> String {
    format!("ERROR: {}", message)
}
