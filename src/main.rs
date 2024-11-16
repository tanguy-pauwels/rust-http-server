use std::{
    fs::read_to_string,
    io::{ BufReader, Write},
    net::{TcpListener, TcpStream}
};

use std::io::prelude::* ;

fn resolve_url_path(url: &str) -> Vec<String> {
    let mut result = Vec::new();
    match url {
        "/" => {
            result.push(read_to_string("static/index.html").unwrap());
            result.push("200 OK".to_string());
        },
        _ => { 
            match read_to_string(format!("static/{}.html", url)) {
                Ok(content) => {
                    result.push(content);
                    result.push("200 OK".to_string());
                },
                Err(_) => {
                    result.push(read_to_string("static/404.html").expect("Something gone wrong while resolving url path"));
                    result.push("404 Not Found".to_string());
                }   
            }
        }
    };
    result
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines() // Fetch all the lines of the Bufreader struct
        .map(|result| result.unwrap()) // BufReader.lines() is a vec
                                       // so we apply a map to unwrap
                                       // all the lines
        .take_while(|line| !line.is_empty()) // Take only line that are not empty
        .collect(); // fetch the result into a Vec
    
    let request_line: Vec<&str>= http_request[0].split(" ").collect(); // Fetch the first line of
                                                                       // the request "GET url/page" in a Vec format ["GET", "/shop", "HTTP/1.1"], ""
    let url = request_line[1];
    
    let url_path = resolve_url_path(url);
    let body = &url_path[0]; // The html page in a text format
    let header = "Content-Type: text/html";
    let status_code = &url_path[1]; // The status code ex: 200 OK
    let content_lenght = body.len();

    let response = format!(
        "HTTP/1.1 {}\r\n{}\r\nContent-Length: {}\r\n\r\n{}",
        status_code, header, content_lenght, body);

    stream.write(response.as_bytes()).expect("an error occured while writing to the stream!");
}

fn main() {
    println!("Listing on http://127.0.0.1:4221");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
         match stream {
             Ok(stream) => {
                println!("accepted new connection {:?}", stream);
                handle_connection(stream);
                 }

             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
