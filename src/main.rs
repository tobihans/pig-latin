use std::io::{self, Write};

fn main() {
    println!("Hello Pig!\n> Let's discuss in **pig latin**. <");
    let mut discussion: String = String::from("Pig-Latin Discussion Record\n");

    loop {
        print!("$>_ ");
        let _ = io::stdout().flush();

        let mut sentence = String::new();

        io::stdin()
            .read_line(&mut sentence)
            .expect("Failed to read line");
        discussion.push_str("$>_ ");
        discussion.push_str(&sentence);
        discussion.push('\n');

        // pigify sentence
        let words: Vec<&str> = sentence.split(' ').collect();
        let mut reply = String::from("| ");

        for w in words {
            if let Some(pig) = pigify(w) {
                reply.push_str(&pig[..]);
                reply.push(' ');
            }
            else {
                discussion.push_str("Unable-hay o-tay understand-hay your-hay essage-may.\n");
                println!("Unable-hay o-tay understand-hay yourhay essage-may.");
                continue;
            }
        }

        println!("{}", reply);
    }
}

fn pigify(word: &str) -> Option<String> {
    const ENGLISH_VOWELS: [&str; 12] = ["a", "e", "i", "o", "u", "y", "A", "E", "I", "O", "U", "Y"];
    let first_char = word.chars().next()?;
    let first_char_to_str = first_char.to_string();
    let first_char_to_str = &first_char_to_str[..];
    let mut result: String;

    // If we have a consonant
    if !ENGLISH_VOWELS.contains(&first_char_to_str) {
        // Get the index for the second character
        let index = first_char.len_utf8();
        result = String::from((&word[index..]).trim_matches('\n').trim());
        result.push_str(&*format!("-{}{}", first_char_to_str, "ay"));
    } else {
        result = String::from(&*format!("{}-{}", word.trim_matches('\n').trim(), "hay"));
    }

    Some(result)
}
