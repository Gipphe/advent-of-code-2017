use std::iter::FromIterator;
use std::fs::File;
use std::io::prelude::Read;

fn are_words_valid(phrase: &str) -> bool {
    let words: Vec<&str> = phrase.split_whitespace().collect();
    for i in 0..words.len() {
        let curr = words[i];
        for j in (i+1)..words.len() {
            if curr == words[j] {
                return false;
            }
        }
    }
    true
}
fn num_valid_phrases(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        if are_words_valid(&line) {
            count += 1;
        }
    }
    count
}

fn sort_chars_in_word(word: &str) -> String {
        let mut word: Vec<char> = word.chars().collect();
        word.sort_by(|a, b| b.cmp(a));
        let word = String::from_iter(word);
        word
}
fn are_words_anagram_valid(input: &str) -> bool {
    let words: Vec<&str> = input.split_whitespace().collect();
    for i in 0..words.len() {
        let curr = sort_chars_in_word(words[i]);
        for j in (i + 1)..words.len() {
            let other = sort_chars_in_word(words[j]);
            if curr == other {
                return false;
            }
        }
    }
    true
}
fn num_valid_phrases_no_anagrams(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        if are_words_anagram_valid(&line) {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Error reading file");

    let input = input.trim();
    let first = num_valid_phrases(&input);
    let second = num_valid_phrases_no_anagrams(&input);
    println!("First: {}", first);
    println!("Second: {}", second);
}
