use std::io::BufRead;
use std::io::BufReader;
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
	
	let mut file = fs::File::open("./pages/index.html").unwrap();
	let bufferd_file = BufReader::new(&mut file);
	eprintln!("{:#?}", bufferd_file.lines());
	
	
}

fn handel_connection(mut stream: TcpStream) {
	let mut buffer:Vec<u8> = vec![0; 1024];
	stream.read(&mut buffer).expect("Reading Buffer Failed");


	let get_request = b"GET / HTTP/1.1\r\n";
	let post_request = b"POST / HTTP/1.1";

  	let (filename, status_line): (&str, &str) = if buffer.starts_with(get_request) {   	   		
   			("./pages/index.html", "HTTP/1.1 200 OK")   		
	   	} else if buffer.starts_with(post_request) {
			("./pages/post.html", "HTTP/1.1 200 OK")
   		} else {
   			println!("Unaccepted HTTP Request");
   			("./pages/404.html", "HTTP/1.1 404 Not Found")
  	};

  	let file = fs::read_to_string(filename).unwrap();
  	let response = format!(
		"{}\r\nContent-Length{}\r\n\r\n{}",
		status_line,   		
		file.len(),
		file
   	);

   	stream.write(response.as_bytes()).expect("Failed to write to stream");
	stream.flush().expect("Failed to flush");

}

