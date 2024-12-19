use problem_1::{Interact,Subprocess};
use std::io;

fn main(){
    println!("######### OH-MY-SHELL starts! #########");
    loop{
        match Interact::interact(){
            Some(_) => continue,
            None => break,
        };
    }
    println!("Something gone wrong in program, interact() returns None");
}