// mod post;
// use crate::post::Post;
// use crate::post::PostValidator;

// mod test_type;
// use crate::test_type::Comment;
// use crate::test_type::CommentValidator;

fn main() {

    let mut s:String = String::from("Hello");

    let t = " World";

    str_pusher(&mut s, &t);

    println!("{}", s);

}


fn str_pusher(original: &mut String, topush: &str) {
    for i in topush.chars() {
        original.push(i);
    };
}