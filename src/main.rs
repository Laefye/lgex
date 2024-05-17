use language::{analyzer::{analyse, Branch, Model}, parser::Parser, types::{Operator, Unit}};

mod language;

fn main() {
    // 
    let node = Parser::new("¬(((p→q)→r)→(r→(¬p∨q)))".to_string()).parse().unwrap();
    let branch = Branch::from(node);
    let mut models = Vec::<Model>::new();
    analyse(branch, 0, &mut models);
    println!("{}", models.len());

}
