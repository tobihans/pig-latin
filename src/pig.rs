use regex::Regex;

pub fn pigify(sentence: &str) -> Option<String> {
    // Punctuation regexs
    let re = Regex::new(r"(?P<punc>\p{P})").unwrap();
    let re2 = Regex::new(r" (?P<punc>\p{P}) ").unwrap();

    let spaced_puncs = re.replace_all(&sentence[..], " $punc ");
    let words: Vec<&str> = spaced_puncs.split(' ').collect();
    let mut result = String::from("");

    for w in words {
        // Continue if found empty token
        if w.len() == 0 {
            continue;
        }
        let pig = pigify_word(w)?;
        result.push_str(&pig[..]);
        result.push(' ');
    }
    let result = re2.replace_all(&result[..], "$punc").to_string();

    Some(result)
}

fn pigify_word(word: &str) -> Option<String> {
    if is_digit(word) {
        return Some(String::from(word));
    }

    if is_punc(word) {
        return Some(String::from(word));
    }

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
        // Do not place dash if we face a single letter
        if word.len() > 1 {
            result.push_str("-");
        }
        result.push_str(&*format!("{}{}", first_char_to_str, "ay"));
    } else {
        result = String::from(&*format!("{}-{}", word.trim_matches('\n').trim(), "hay"));
    }

    Some(result)
}


fn is_digit(string: &str) -> bool {
    let re = Regex::new(r"^\d+$").unwrap();
    re.is_match(string)
}

fn is_punc(string: &str) -> bool {
    let re = Regex::new(r"^\p{P}+$").unwrap();
    re.is_match(string)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_digit() {
        assert!(is_digit("1222"));
        assert!(!is_digit("12d22"));
    }

    #[test]
    fn it_is_punc() {
        assert!(is_punc("{}"));
        assert!(!is_punc("aii."));
    }

    #[test]
    fn it_pigify_word() {
        assert_eq!(pigify_word("Person"), Some(String::from("erson-Pay")));
        assert_eq!(pigify_word("Eye"), Some(String::from("Eye-hay")));
        assert_eq!(pigify_word(""), None);
    }

    #[test]
    fn it_pigify() {
        assert_eq!(pigify("The quick fox jumps over the crazy dog."),
                   Some(String::from("he-Tay uick-qay ox-fay umps-jay over-hay he-tay razy-cay og-day."))
        );
        assert_eq!(pigify("It's going to work!"),
                   Some(String::from("It-hay'say oing-gay o-tay ork-way!"))
        );
    }
}
