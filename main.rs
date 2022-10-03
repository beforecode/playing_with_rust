use std::io;

fn main() {
	println!("Please input something");	
	let mut input = String::new();
	
	io::stdin().read_line(&mut input)
	.expect("Task Failed");
	input.pop();

	println!("Your input is {}", input);	
	let quite = b"q";

	// let booly = assert_eq!(input.as_bytes(), quite);
	
	if input.as_bytes() == quite {
		println!("Match");		
	} else {
		println!("No Match");
	}

}