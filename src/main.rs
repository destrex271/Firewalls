use std::{
    fs,
    thread,
    time::Duration,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use tcp::ThreadPool;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let thread_pool = ThreadPool::build(4); 
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();
    println!("Request: {:#?}", req_line);
    let (stat_line, file_name) = if req_line == "GET / HTTP/1.1"{
        ("HTTP/1.1 200 OK", "test.html")
    }else if req_line == "GET /sleep HTTP/1.1"{
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "test.html")
    }else if req_line == "POST / HTTP/1.1"{
        ("HTTP/1.1 201 OK", "post.html")
    }else if req_line == "PUT / HTTP/1.1"{
        ("HTTP/1.1 201 OK", "put.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(file_name).unwrap();
    let len = contents.len();
    let response = format!("{stat_line}\r\nContent-Length: {len}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
