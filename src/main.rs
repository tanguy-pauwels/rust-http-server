use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream}
};

use std::io::prelude::* ;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines() // Fetch all the lines of the Bufreader struct
        .map(|result| result.unwrap()) // BufReader.lines() is a vec
                                       // so we apply a map to unwrap
                                       // all the lines
        .take_while(|line| !line.is_empty()) // Take only line that are not empty
        .collect(); // fetch the result into a Vec
    let request_line: Vec<&str>= http_request[0].split(" ").collect();

    let status_code = match request_line[0] {
            "GET" => {
                match request_line[1] {
                    "/" => "200 OK",
                    _ => "404 Not Found"
                }
            },
            "POST" => {
                match request_line[1] {
                    "/" => "200 OK",
                    _ => "404 Not Found"
                }
            }
            _ => "404 Not Found",
    };
    let header = "Content-Type: text/html";
    let body = r#"
    <!DOCTYPE html>
    <html>
        <head> 
            <title> Hello World ! </title>
        </head>
        
        <body> 
            <h1> Hello world ! <h1> 
            <p> This is my first basic http server based on rust </p> 
        </body>
    </html>
        "#;
    let content_lenght = body.len();
    let response = format!(
        "HTTP/1.1 {}\r\n{}\r\nContent-Length: {}\r\n\r\n{}",
        status_code, header, content_lenght, body);

    stream.write(response.as_bytes()).expect("an error occured !");
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
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
