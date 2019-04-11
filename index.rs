// ported from https://github.com/WillSen/spellchecker-autocorrect

extern crate regex;
use regex::Regex;

extern crate serde;
extern crate serde_json;

use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*; // necessary for File type to have .write_all() method
use std::path::Path;
use std::collections::HashMap;



fn get_words(corpus: &str) -> HashMap<String, i32> {
    if !Path::new("corpus_count.txt").exists() {
        // can't get rid of this, because it owns the strings the vector later references
        let corpus = read_to_string(Path::new(corpus))
                    .unwrap()
                    .to_lowercase();
        let re = Regex::new(r"(?P<word>[A-Za-z]+)").unwrap();
        let mut hash: HashMap<String, i32> = HashMap::new();
        for word in re.captures_iter(&corpus) { //.unwrap();
            // inserts a 0 if entry doesn't exist, then increments by 1.
            // ergo, if entry present, then incremented by 1.
            // if not present, then inserted 0, then incremented by 1.
            *hash.entry(word["word"].to_string().clone()).or_insert(0) += 1;
        }
        match File::create(Path::new("corpus_count.txt")) {
            Ok(mut x) => {
                x.write_all(
                    serde_json::to_string(&hash)
                    .ok()
                    .unwrap()
                    .as_bytes()
                ).ok();
             },
            _ => { println!("Couldn't create new file."); }
        };
        hash
    } else {
        let corpus_json = read_to_string(Path::new("corpus_count.txt")).unwrap();
        let hash: HashMap<String, i32> = serde_json::from_str(&corpus_json).ok().unwrap();
        hash
    }
}

fn edit_distance1(word: &str) -> Vec<String> {
    let word_lowercase = word.to_lowercase();
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let word_coll = word.chars();
    let mut results: Vec<String> = Vec::new();

    // all permutations of word with 1 char extra
    // add 1 because enumerate only iterates the length
    // of the array, and so the insert stops 1 index short
    for i in 0..(word_coll.count() + 1) {
        for j in alphabet.to_string().chars() {
            let mut new_word: Vec<char> = word_lowercase.chars().collect::<Vec<char>>();
            new_word.insert(i, j);
            results.push(new_word.into_iter().collect::<String>());
        }
    }


    //Removing any one character from the word.
    if word_lowercase.len() > 1 {
      for i in 0..word_lowercase.len() {
        let mut new_word: Vec<char> = word_lowercase.chars().collect::<Vec<char>>();
        new_word.remove(i);
        results.push(new_word.into_iter().collect::<String>());
      }
    }

    //Transposing (switching) the order of any two adjacent characters in a word.
    if word_lowercase.len() > 1 {
      for i in 0..(word_lowercase.len() - 1) {
          let mut new_word: Vec<char> = word_lowercase.chars().collect::<Vec<char>>();
          let r = new_word.remove(i);
          new_word.insert(i + 1, r);
          results.push(new_word.into_iter().collect::<String>());
      }
    }

    //Substituting any character in the word with another character.
    for i in 0..word_lowercase.len() {
        for j in alphabet.chars() {
            let mut new_word: Vec<char> = word_lowercase.chars().collect::<Vec<char>>();
            new_word[i] = j;
            results.push(new_word.into_iter().collect::<String>());
        }
    }

    results
}


fn correct (dictionary: HashMap<String, i32>, word: &str) -> String {
    // The function only continues on to the rest if the `word` is not present in dictionary
    if let Some(_x) = dictionary.get(word) {
        return word.to_string();
    }

    let mut max_count = 0;
    let mut correct_word = word.to_string();
    let edit_distance1_words = edit_distance1(word);
    let mut edit_distance2_words: Vec<String> = Vec::new();


    // Generate list of words that are 2 errors removed from `word`
    for i in edit_distance1_words.clone() {
        edit_distance2_words.append(&mut edit_distance1(i.as_str()));
    }

    // Find most frequently used permutation of `word` at error distance 1
    for i in edit_distance1_words.clone() {
        if let Some(x) = dictionary.get(&i) {
            if *x > max_count {
                max_count = *x;
                correct_word = i;
            }
        }
    }

    let mut max_count2 = 0;
    let mut correct_word2 = correct_word.clone();

    // Find the most frequently used permutation of `word` at error distance 2
    for i in edit_distance2_words {
        if let Some(x) = dictionary.get(&i) {
            if *x > max_count2 {
                max_count2 = *x;
                correct_word2 = i;
            }
        }
    }

    // pick the most probable correction from between correct_word1 or correct_word2
    if word.len() < 6 {
        if max_count2 > (100 * max_count) {
            return correct_word2
        }
        return correct_word;
    } else {
        if max_count2 > (4 * max_count) {
            return correct_word2
        }
        return correct_word;
    }
}

pub fn suggest (dir: &str, word: &str) -> String {
    correct(get_words(dir), word)
}
