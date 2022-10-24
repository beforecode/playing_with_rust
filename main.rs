// mod post;
// use crate::post::Post;
// use crate::post::PostValidator;

mod test_type;
use crate::test_type::Comment;
use crate::test_type::CommentValidator;


pub fn main() {

    // let post = Post {
    //     title: String::from("my title"),
    //     content: String::from("my content"),
    //     public: true
    // };

    // println!("{}", post.length());

    let comm = Comment {
        content: String::from("Hello Wold"),
        author: String::from("Yassine"),
        likes: 0,
    };

    check_length(comm);

}

fn check_length(c: Comment) {
    println!("{}", c.content);
}