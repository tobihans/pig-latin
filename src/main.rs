mod pig;

use std::io::{self, Write};
use regex::Regex;

fn main() {
    let re = Regex::new("^(?P<sentence>.*)\n$").unwrap();
    println!("Hello Pig!\n> Let's discuss in **Pig Latin**. <");

    loop {
        print!("$>_ ");
        let _ = io::stdout().flush();

        let mut sentence = String::new();

        io::stdin()
            .read_line(&mut sentence)
            .expect("Failed to read line");

        // pigify sentence
        let sentence = re.replace_all(&sentence, "$sentence").to_string();
        let mut reply = String::from("| ");
        if let Some(s) = pig::pigify(&sentence) {
            reply.push_str(&s);
        } else {
            reply.push_str("I-hay an-cay ot-nay understand-hay):");
        }
        println!("{}", reply);
    }
}
