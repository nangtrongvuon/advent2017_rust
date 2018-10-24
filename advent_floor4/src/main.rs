use std::fs::File;
use std::error::Error;
use std::process;
use std::io::prelude::*;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let mut contents = String::new();

    match read_file_to_content("input.txt") {
        Ok(read_contents) => contents = read_contents,
        Err(e) => {
            eprintln!("Error {}", e);
            process::exit(1);
        },
    }
    
    println!("{}", check_valid_passphrase_count(&contents));
    println!("{}", check_valid_anagram_passphrase_count(&contents));
}

fn read_file_to_content(filename: &str) -> Result<String, Box<Error>> {

    let mut contents = String::new();
    let mut input = File::open(filename).expect("File not found");
    input.read_to_string(&mut contents)?;

    Ok(contents)
}

// Go through each line, check for duplicates by caching found phrases in a hashmap.
fn check_valid_passphrase_count(input: &str) -> usize {
    let mut valid_count = 0;
    
    'passphrase_check: for passphrase in input.lines() {
        let mut passphrase_check_map: HashMap<String, usize> = HashMap::new();
        let passphrase_vec: Vec<String> = passphrase.split(" ").map(|s| s.to_string()).collect();

        'phrase_check: for phrase in passphrase_vec {
            let phrase_count = passphrase_check_map.entry(phrase).or_insert(0);
            *phrase_count += 1; 
            if *phrase_count > 1 {
                continue 'passphrase_check;
            }
        }
        valid_count += 1;
    }   

    return valid_count;
}

// Go through each lines, check for anagrams by sorting the string.
fn check_valid_anagram_passphrase_count(input: &str) -> usize {
    let mut valid_count = 0;
    
    'passphrase_check: for passphrase in input.lines() {
        let mut passphrase_check_map: HashMap<String, usize> = HashMap::new();
        let passphrase_vec: Vec<String> = passphrase.split(" ").map(|s| s.to_string()).collect();

        'phrase_check: for phrase in passphrase_vec {
            let mut phrase_chars: Vec<char> = phrase.chars().collect();
            phrase_chars.sort_by(|a, b| a.cmp(b));

            let sorted_phrase = String::from_iter(phrase_chars); 
            
            let phrase_count = passphrase_check_map.entry(sorted_phrase).or_insert(0);
            *phrase_count += 1; 
            if *phrase_count > 1 {
                continue 'passphrase_check;
            }
        }
        valid_count += 1;
    }   

    return valid_count;
}