pub struct Post {
	pub title: String,
	pub content: String,
	pub public: bool
}

pub trait PostValidator {
	fn length(&self) -> &str;
	fn is_public(&self) -> bool;
}

impl PostValidator for Post {
	
	fn length(&self)  -> &str{
		return "Hello World"
	}

	fn is_public(&self) -> bool {
		if self.public {
			return true
		} return false
	}
}
