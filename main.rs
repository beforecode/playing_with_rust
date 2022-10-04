use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn main() {
	let server = TcpListener::bind("127.0.0.1:8080").unwrap();
	println!("TCP Server Is OK ON Port 8080");

	for stream in server.incoming() {
		println!("Connection Established");
		
		// println!("{:?}", &stream);

		let stream = stream.unwrap();
		handel_connection(&stream);
	}	
	

}

fn handel_connection(mut stream: &TcpStream) {
	let mut buffer: Vec<u8> = vec![0;1024];
	stream.read(&mut buffer).unwrap();
	// println!("{:?}", &buffer);

	// let get_request: [u8; 11] = *b"Get request";
	
}