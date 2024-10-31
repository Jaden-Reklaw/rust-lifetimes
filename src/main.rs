fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;
    for language in languages {
        if found {
            return language;
        }
        if language == current {
            found = true;
        }
    }
    
    languages.last().unwrap()
}

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

fn main() {
    let languages = vec![
        "Rust".to_string(),
        "Python".to_string(),
        "JavaScript".to_string(),
        "Java".to_string(),
        "C++".to_string()
    ];

    let result = next_language(&languages, "JavaScript");
    println!("Next language is {}", result);

    let last_language = last_language(&languages);
    println!("Last language is {}", last_language);

    let longest = longest_language("JavaScript", "Python");
    println!("Longest language is {}", longest);
}
