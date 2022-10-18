mod post;
use crate::post::Post;
use crate::post::PostValidator;


pub fn main() {

    let post = Post {
        title: String::from("my title"),
        content: String::from("my content"),
        public: true
    };

    println!("{}", post.length());
}