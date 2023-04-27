use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};
/** 
 * A simple TCP server that listens to a port and prints the stdin it recieves
 */

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Not able to bind :(");
    for stream in listener.incoming() {
        match stream {
            Err(error) => { eprintln!("Error in stream: {}", error)}
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
                });
            }
        }
    } 
}
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Inbound connection address: {}", stream.peer_addr()?);
    let mut buf = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()) }
        stream.write(&buf[..bytes_read])?;
    }
}

