// mod args;
mod nepse;
use std::env;

// use args::Args;

fn main() {
      let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("you are breaking the law by scraping the Nepse Data.but if you still wanna continue then please provide the Nepse Symbol as the argument");
    }

    
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
