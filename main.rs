use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;
use std::fs;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	println!("Server is OK on port 7878");
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		handel_connection(stream);
	}
	
}

fn handel_connection(mut stream: TcpStream) {
	let mut buffer:Vec<u8> = vec![0; 1024];
	stream.read(&mut buffer).expect("Reading Buffer Failed");
	
	let get_request = b"GET / HTTP/1.1\r\n";

   if buffer.starts_with(get_request) {
   	
   	let file = fs::read_to_string("./pages/index.html").expect("Faild to read Inedx File");
   	let status_line = "HTTP/1.1 200 OK";
   	
   	let response = format!(
   		"{}\r\nContent-Length{}\r\n\r\n{}",
   		status_line,   		
   		file.len(),
   		file);


   	stream.write(response.as_bytes()).expect("Failed to write to stream");
   	stream.flush().expect("Failed to flush");

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