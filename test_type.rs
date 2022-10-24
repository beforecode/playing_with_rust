pub struct Comment {
	pub content: String,
	pub author: String,
	pub likes: u32,
}

pub trait CommentValidator {
	fn length(&self) -> usize ;
}

impl CommentValidator for Comment {
	fn length(&self) -> usize {
		let x = self.content.len();
		x
	}
}