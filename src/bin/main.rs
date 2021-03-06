use std::fs;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use RustServer::ThreadPool;
fn main() {
    
    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:7878").unwrap(); 
    
    for stream in listener.incoming().take(3){
        let stream = stream.unwrap();

        let pool = ThreadPool::new(10);



        pool.execute(||{ handle_connection(stream);
    })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let ( status_line, filename) = 
    
    if buffer.starts_with(get) {          
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };
    
    let content = fs::read_to_string(filename).unwrap();
    
    let response = format!("{}\r\nContent-Lenght: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content);
        
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        
    
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]))

}
 