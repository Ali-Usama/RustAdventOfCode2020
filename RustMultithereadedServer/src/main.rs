use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        // println!("Connection Established");
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    // adds buffering by managing calls to the Std::io::Read trait method for us
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();

    // collect the lines that the browser sends to our server
    // let http_requests: Vec<_> = buf_reader.lines()
    //     .map(|result| result.unwrap())
    // browser signals the end of an HTTP request by sending two newline characters in a row,
    // so to get one stream, we take lines until we get an empty line
    // .take_while(|line| !line.is_empty())
    // .collect();

    // println!("Request: {:#?}", http_requests);   // print using the pretty debug formatter
}