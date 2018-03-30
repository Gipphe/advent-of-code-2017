use std::iter::FromIterator;

mod input;

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

pub fn main() {
    let first = num_valid_phrases(&input::INPUT);
    let second = num_valid_phrases_no_anagrams(&input::INPUT);

    assert_eq!(first, 466, "Day 4-1 is incorrect: {}", first);
    assert_eq!(second, 251, "Day 4-2 is incorrect: {}", second);
    println!("Day 4-1: {}", first);
    println!("Day 4-2: {}", second);
}
