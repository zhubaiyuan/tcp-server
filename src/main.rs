use std::net::{TcpListener, TcpStream};
use std::thread;
use rand::Rng;
use std::time::Duration;
use std::io::{Read, Write, Error};

// handles a single client
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    // a buffer holds data temporarily
    let mut buf = [0; 512];
    // reads all data in the stream
    loop {
        // read method returns the length of the data it has read
        // ? unwraps the result to an Ok if everything was fine; otherwise, it does an early return of the error
        let bytes_read = stream.read(&mut buf)?;
        // if reaches the end of the stream, the read returns a zero
        if bytes_read == 0 {
            return Ok(());
        }
        // thread_rng selects an integer between 0 and 5 randomly
        let sleep = Duration::from_secs(rand::thread_rng().gen_range(0, 5));
        println!("Sleeping for {:?} before replying", sleep);
        // sleeps for the randome time duration using std::thread::sleep
        std::thread::sleep(sleep);
        // writes the same data back to the stream with the slice
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    // TcpListenner represents a TCP socket that is listoning for incoming connections from client
    // bind the local IP and port pair to create a local listening socket
    // expect returns the listener if there is no errors, otherwise, it panics with the error message
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Could not bind");
    // incoming returns an iterator over streams that have connected to the server
    for stream in listener.incoming() {
        match stream {
            // eprintln! outputs errors
            Err(e) => { eprintln!("failed: {}", e); }
            Ok(stream) => {
                // spawns a worker thread to handle each client connection
                // a move closure reads a vairable stream from the enclosing scope
                thread::spawn(move || {
                    // reads from each stream and writes it back
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
