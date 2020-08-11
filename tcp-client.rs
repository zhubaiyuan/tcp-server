use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;
use std::net::SocketAddr;

fn main() {
    let remote: SocketAddr = "127.0.0.1:8888".parse().unwrap();
    // wraps the Duration in a Some before passing in
    let mut stream = TcpStream::connect_timeout(&remote, Duration::from_secs(1)).expect("Could not connect to server");
    // uses set_read_timeout to set the timeout to 4s, the client will abort the connection
    stream.set_read_timeout(Some(Duration::from_secs(4))).expect("Could not set a read timeout");
    loop {
        // initializes an empty string to read user input locally
        let mut input = String::new();
        // initializes a vector of u8 to read responses from the server
        let mut buffer: Vec<u8> = Vec::new();
        // reads a line from standard input and stores it in the input
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        // writes to the connection as a stream of bytes
        stream.write(input.as_bytes()).expect("Failed to write to server");
        // reads the response from the server using BufReader
        let mut reader = BufReader::new(&stream);
        // read_until method reads the data in the buffer, which grows as needed
        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        // prints out the buffer as a string
        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
    }
}
