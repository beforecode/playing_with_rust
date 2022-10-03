use std::net::{TcpListener, TcpStream};

fn main() {
	let server = TcpListener::bind("127.0.0.1:8080").unwrap();
	println!("TCP Server Is OK ON Port 8080");
	for stream in server.incoming() {
		println!("Connection Established");
		let stream = stream.unwrap();
		handel_connection(stream);
	}	
}

fn handel_connection(stream: TcpStream) {
	println!("{:?}", stream);
}