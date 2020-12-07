use num_format::{Locale, ToFormattedString};
use std::thread;
use std::time::Duration;

fn hash_md5(input: String) -> String {
    format!("{:x}", md5::compute(input.into_bytes()))
}

fn main() {
    let goal_hash = "dbba1bfe930d953cabcc03d7b6ab05e";

    let length = 17;

    let alphabet = "bdeilmost-"
        .split("")
        .map(String::from)
        .filter(|letter| !letter.is_empty())
        .collect::<Vec<String>>();

    let mut words = alphabet.clone();

    for phase in 1..length {
        let mut temp: Vec<String> = Vec::new();

        #[cfg(not(debug_assertions))]
        let loopable_words = words.iter();

        #[cfg(debug_assertions)]
        let loopable_words = words.iter().enumerate();

        for data in loopable_words {
            #[cfg(not(debug_assertions))]
            let word = data;

            #[cfg(debug_assertions)]
            let word = data.1;
            #[cfg(debug_assertions)]
            let index = data.0;

            for letter in alphabet.iter() {
                let new_word = format!("{}{}", word, letter);

                temp.push(new_word);
            }

            #[cfg(debug_assertions)]
            if index != 0 && ((index % 1000000) == 0) {
                println!(
                    "Completed phase {}/{}'s sub-phase {}/{}",
                    phase,
                    length,
                    index.to_formatted_string(&Locale::en),
                    words.len().to_formatted_string(&Locale::en),
                );
            }
        }

        words = temp;
    }

    let length = words.len();
    println!("\n\nLength of words: {}\n\n", length);

    thread::sleep(Duration::from_secs(2));

    #[cfg(not(debug_assertions))]
    let loopable_words = words.iter();

    #[cfg(debug_assertions)]
    let loopable_words = words.iter().enumerate();

    for data in loopable_words {
        #[cfg(not(debug_assertions))]
        let word = data;

        #[cfg(debug_assertions)]
        let word = data.1;
        #[cfg(debug_assertions)]
        let attempts = data.0;

        let merged = format!(
            "{}{}",
            word, "........................................................!1"
        );

        let hash = hash_md5(hash_md5(hash_md5(hash_md5(merged.clone()))));

        #[cfg(debug_assertions)]
        println!(
            "Attempts: {}/{}, Hash: {}, Text: {}",
            attempts, length, hash, merged
        );

        if hash == goal_hash {
            println!("We found the password: {}", merged);

            return;
        }
    }
}
