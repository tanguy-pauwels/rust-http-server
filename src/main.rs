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

    let response = match request_line[0] {
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
    println!("{:?}", response);

    stream.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("200\n");
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
