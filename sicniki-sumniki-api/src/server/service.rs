use serde::{Serialize};

use crate::{contains_czs, fix_case, contains_czs_plus_with_caron};
use crate::Repository;

pub struct Service<'a> {
    repo: &'a Repository
}

impl<'a> Service<'a> {
    pub fn new(repo: &Repository) -> Service {
        Service {
            repo
        }
    }

    pub fn get_word_data(&self, word: &str) -> std::result::Result<Word, Box<dyn std::error::Error>> {
        let similar_words: Vec<String> = self.repo.get_similar_words(word)?
            .iter()
            .map(|w| fix_case(w, word))
            .collect();

        Ok(Word {
            word: word.to_string(),
            similar_words
        })
    }

    pub fn save_word(&self, word: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        self.repo.insert_new_word(word)
    }

    pub fn fix_text(&self, text: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
        let mut fixed_text = String::from(text);

        let words: Vec<&str> = text.split_whitespace()
            .map(|s| s.trim_matches(|c| !char::is_alphanumeric(c)))
            .collect();

        let mut processed_words: Vec<&str> = vec![];

        for word in words.iter() {
            if !contains_czs(word) {
                continue
            }

            let lowercase_word = word.to_lowercase();
            if processed_words.contains(word) {
                continue
            }

            let similar_words = self.repo.get_similar_words(&lowercase_word)?;
            if similar_words.len() != 1 {
                continue
            }

            if let Some(correct_word) = similar_words.get(0) {
                let correct_word = fix_case(correct_word, &word);
                fixed_text = fixed_text.replace(word, correct_word.as_ref());
                processed_words.push(word)
            }
        }

        Ok(fixed_text)
    }

    pub fn confirm_text(&self, text: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let words: Vec<String> = text.split_whitespace()
            .map(|s| s.trim_matches(|c| !char::is_alphanumeric(c)).to_lowercase())
            .filter(|s| contains_czs_plus_with_caron(s))
            .collect();

        for word in words {
            self.save_word(&word)?;
        }

        Ok(())
    }
}

#[derive(Serialize, Debug)]
pub struct Word {
    word: String,
    similar_words: Vec<String>
}