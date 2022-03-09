// mod args;
extern crate requestty;
use requestty::{Question,prompt_one };
mod nepse;
use std::env;

// use args::Args;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        interactive_mode();
    }else {
        noninteractive_mode();
        println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    }

    
}


fn interactive_mode() {
    
    let question = Question::expand("overwrite")
    .message("Conflict on `file.rs`")
    .choices(vec![
        ('y', "Overwrite"),
        ('a', "Overwrite this one and all next"),
        ('d', "Show diff"),
    ])
    .default_separator()
    .choice('x', "Abort")
    .build();
    let answer = prompt_one(question);
println!("{:#?}",answer);
    println!("interactive mode");
}

fn noninteractive_mode() {
    println!("noninteractive mode");
}