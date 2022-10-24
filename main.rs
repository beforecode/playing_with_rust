// mod post;
// use crate::post::Post;
// use crate::post::PostValidator;

// mod test_type;
// use crate::test_type::Comment;
// use crate::test_type::CommentValidator;

fn main() {

    let mut s:String = String::from("Hello");


    {
        s.push_str(" World");

    }

    println!("{}", s);
}



