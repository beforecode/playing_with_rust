pub struct Comment {
	pub content: String,
	pub author: String,
	pub likes: u32,
}

pub trait CommentValidator {
	fn length(content: &Self) -> bool;
}

impl CommentValidator for Comment {
	fn length(content: &Self) -> bool {
		println!("{}", content.content);
		content.content.len() > 0
	}
}