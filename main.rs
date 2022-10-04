use std::net::{TcpListener, TcpStream};
use std::io::Read;
// use std::fs;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	println!("Server is Ok");
	let x = "GET / HTTP/1.1\r\n".as_bytes();
	println!("{:?}", x);
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		handel_connection(stream);
	}
}

fn handel_connection(mut stream: TcpStream) {
	// let mut buffer : Vec<u8> = vec![0; 1024];
	// stream.read(&mut buffer).unwrap();
	// let get_request = b"GET / HTTP/1.1\r\n";

	let mut buffer:Vec<u8> = vec![0; 1024];
	stream.read(&mut buffer).expect("Reading Buffer Failed");
	// println!("{:?}", &buffer);
   let get_request = b"GET / HTTP/1.1\r\n";
   
   if buffer.starts_with(get_request) {
   	println!("1");
   } else {
   	println!("Unaccepted HTTP Request");
   }


	// let contents = fs::read_to_string(filename).unwrap();
	// let response = format!(
	// 	"{}\r\nContent-Length{}\r\n\r\n{}", 
	// 	status_line,
	// 	contents.len(),
	// 	contents
	// 	);
	// stream.write(response.as_bytes()).unwrap();
	// stream.flush().unwrap();
}