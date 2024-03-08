use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();

    let request = String::from_utf8_lossy(&buf[..]);
    println!("Request recived. Content: {}", request);

    let response = match &request[..2] {
        "1" => throw_text_file("/home/hal9000/GOPHER-BAR/docs/doc1.txt"),
        "0" => throw_directory("."), // throws current dir
        _ => throw_error("ERROR: Invalid request!"),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn throw_text_file(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Ok(content) => format!("i{}{}\t{}\t{}\r\n", "text/plain", file_path, content, ""),
        Err(_) => throw_error("ERROR: File not found!"),
    }
}

fn throw_directory(directory_path: &str) -> String {
    let mut response = String::new();
    match fs::read_dir(directory_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name().into_string().unwrap();
                    let file_type = if entry.file_type().unwrap().is_dir() {"1"} else {"0"};
                    response.push_str(&format!("{}{}\t{}\t{}\r\n", file_type, file_name, file_name, ""));
                }
            }
        }
        Err(_) => return throw_error("ERROR: Failed to read directory!"),
    }
    response
}

fn throw_error(message: &str) -> String {
    format!("3{}\t{}\t{}\r\n", message, "", "")
}

fn main() {
    println!("INITIALIZING...");
    let ip = "10.0.0.234";
    let port = "7070";
    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(address).unwrap();
    println!("INITIALIZED; SERVER LISTENING ON PORT {} AT IP ADDRESS {}...", port, ip);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
            }
        }
    }

}