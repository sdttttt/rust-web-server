use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::fs;

use webserver::MultiThreading::ThreadPool;

fn main() {

    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection !");

        pool.execute(move || {
            connection_handler(stream);
        })

    }

}

fn connection_handler(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

}
