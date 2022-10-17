use std::io::BufRead;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	println!("Server is OK on port 7878");
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		handel_connection(stream);
	}
	
}

fn handel_connection(mut stream: TcpStream) {
let mut buffer: Vec<u8> = vec![0; 1024];
	stream.read(&mut buffer).unwrap();

	let mut reader = BufReader::new(stream);
	let mut line = String::new();
	let x = reader.read_line(&mut line).unwrap();


	println!("{:?}", x);
}

