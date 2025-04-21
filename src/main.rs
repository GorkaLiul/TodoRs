#[allow(dead_code)]
#[allow(unused_variables)]
//use std::env::args;
//use clap;
use todo::task::*;

fn main() {
let mut todo = TaskList::default();
todo.add("eat cereal");
todo.add("code!");
todo.add("be happy"); 
todo.display();


 todo.rem("code!");
todo.display();


}

