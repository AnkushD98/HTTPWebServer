use std::{
    fs,
    net::{TcpListener,TcpStream},
    io::{prelude::*,BufReader},
    thread,
    time::Duration
};

use http_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);

    /*
    In case printing the request is needed, uncomment this code and comment out line 28.

    let http_request: Vec<_> = buf_reader
                       .lines()
                       .map(|result| result.unwrap())
                       .take_while(|line| !line.is_empty())
                       .collect();
    println!("Request is: {:#?}", http_request);
    */

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    respond_to_request(stream,request_line);
}

fn respond_to_request(mut stream: TcpStream,request_line: String){
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

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
