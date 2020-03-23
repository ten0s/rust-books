extern crate ctrlc;
use std::fs;
//use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    /*
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");
    */

    let pool = ThreadPool::new(4);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    /*
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    */

    for stream in listener.incoming() {
        /*
        if !running.load(Ordering::SeqCst) {
            println!("Got Ctrl-C. Exiting...");
            break;
        }
        */

        match stream {
            Ok(tcp_stream) => {
                pool.execute(|| {
                    handle_connection(tcp_stream);
                });
            }
            Err(e) => {
                panic!("IO error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let http11 = "HTTP/1.1";
    let ok = "200 OK";
    let not_found = "404 NOT FOUND";

    let index = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(index) {
        (ok, "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (ok, "index.html")
    } else {
        (not_found, "404.html")
    };

    let body = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{http} {status}\r\n\r\n{body}",
        http = http11,
        status = status,
        body = body
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
