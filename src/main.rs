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

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // let (status_line, file_name) = if buffer.starts_with(get) {
    //     ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    // };

    // let contents = fs::read_to_string(file_name).unwrap();
    // println!("{}", contents);
    // let response = format!("{} {}", status_line, contents);

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();

}
