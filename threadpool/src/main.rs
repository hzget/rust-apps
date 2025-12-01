//!
//! Requirment:
//!    A web server to serve http request
//!
//! EndPoint:
//!    /         -> ("HTTP/1.1 200 OK", "hello.html")
//!    /sleep    -> sleep 5 seconds and then ("HTTP/1.1 200 OK", "hello.html")
//!    otherwise -> ("HTTP/1.1 404 NOT FOUND", "404.html")

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use std::thread;
use std::time::Duration;
use threadpool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let pool = threadpool::ThreadPool::new(4);
    for stream in listener.incoming().take(6) {
        let stream = stream.unwrap();
        let task = || handle_connection(stream);
        pool.execute(task);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
