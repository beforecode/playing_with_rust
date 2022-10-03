// use std::net::TcpListener;

fn main() {
	let _server = TcpListener::bind("127.0.0.1:8080")
	.expect("Task Failed");
	println!("Server Is Running On Port 8080");
}