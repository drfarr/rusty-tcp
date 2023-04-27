use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};

/**
 * A utility that emulates Netcat, allowing you to sending raw data over a network connection.
 */
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to port");

    loop{
         let mut input = String::new();
         let mut buffer: Vec<u8> = Vec::new();
         io::stdin().read_line(&mut input).expect("Could not read from stdin");
         stream.write(input.as_bytes()).expect("Could not write to server");

         let mut reader = BufReader::new(&stream);

         reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
         print!("{}", str::from_utf8(&buffer).expect("Could not write to buffer as a string"))
    }
}