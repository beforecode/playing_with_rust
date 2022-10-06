// use std::io::BufRead;
// use std::io::BufReader;
// use std::net::{TcpListener, TcpStream};
// use std::io::Write;
// use std::io::Read;
// use std::fs;
// ________________

mod dt;
fn main() {
	// let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	// println!("Server is OK on port 7878");
	// for stream in listener.incoming() {
	// 	let stream = stream.unwrap();
	// 	handel_connection(stream);
	// }	

	dt::run();


}

// fn handel_connection(mut stream: TcpStream) {
	// let stream_buf = BufReader::new(&mut stream);
	// let res = stream_buf
	// .lines()
	// .map(|resault| resault.expect("Task Failed Due To Underlying OS") )
	// .take_while(|item| !item.is_empty())
	// .collect::<Vec<_>>();

	// println!("{:#?}", &stream_buf.lines());
// }

