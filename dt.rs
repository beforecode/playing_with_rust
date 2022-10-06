pub fn run() {
	pub struct Movie {
		pub title: String,
		pub release: u32,
		pub director: String
	}

	println!("xxx");

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

	// Details::age(&titanic_movie);
	// Details::directors_name(&titanic_movie);

	 println!("{}", titanic_movie.age());

}