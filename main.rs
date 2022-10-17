struct Article {
    title: String,
    content: String
}
struct Comment {
    content: String
}

pub trait Summerize {
    fn summed(&self);
}

pub trait CValidator {
    fn is_emmpty(&self);
}

impl Summerize for Article {
    fn summed(&self) {
        println!("The title of this article is {} and content is {}", self.title, self.content);
    }
}

impl CValidator for Comment {
    fn is_emmpty(&self) {
        if self.content.len() > 0 {
            println!("Allow to be published");
        } else {

            println!("Trigger a warning");
        }
    }
}


pub fn main() {
    let c = Comment {
        content: String::from("")
    };
    c.is_emmpty();
}