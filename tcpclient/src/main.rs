// TCP Client
// Dru Delarosa | @dntstck
// Simple Rust TCP Client

use std::io::{Read,Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Up".as_bytes()).unwrap();
    let mut buffer = [0, 5];
    stream.read(&mut buffer).unwrap();
    println!("Server:{:?}:", str::from_utf8(&buffer).unwrap());
    
}
