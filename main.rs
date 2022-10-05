// use std::io::BufRead;
// use std::io::BufReader;
// use std::net::{TcpListener, TcpStream};
// use std::io::Write;
// use std::io::Read;
// use std::fs;
// ________________

fn main() {
	// let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	// println!("Server is OK on port 7878");
	// for stream in listener.incoming() {
	// 	let stream = stream.unwrap();
	// 	handel_connection(stream);
	// }	

	pub struct Movie {
		pub title: String,
		pub release: u32,
		pub director: String
	}

	trait Details {
		fn age(&self) -> u32;
		fn directors_name(&self) -> &str;
	}

	impl Details for Movie {
		fn age(&self) -> u32 {
			let age:u32 = 2022 - &self.release;
			println!("This movie was released {} years age", age);
			age
		}
		fn directors_name(&self) -> &str {
			let dname = &self.director;
			println!("The name of the director is {}", dname);
			dname
		}
	}

	let titanic_movie = Movie {
		title: String::from("Titanic"),
		release: 1992,
		director: String::from("Yassine")
	};

	Details::age(&titanic_movie);
	Details::directors_name(&titanic_movie);

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

