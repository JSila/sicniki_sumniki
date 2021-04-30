use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind::UniqueViolation, Error::DatabaseError};
use itertools::Itertools;

use crate::establish_connection;
use crate::util::*;

#[derive(Queryable)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub sicnik: String,
}

#[derive(Insertable)]
#[table_name = "words"]
pub struct NewWord {
    pub word: String,
    pub sicnik: String,
}

diesel::table! {
    words {
        id -> Integer,
        word -> Text,
        sicnik -> Text,
    }
}

pub fn get_similar_words(word: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let sicnik = convert_to_czs(word).to_lowercase();

    let connection = establish_connection();

    Ok(words::table
        .filter(words::sicnik.eq(&sicnik))
        .load::<Word>(&connection)?
        .iter()
        .map(|w| fix_case(&w.word, word))
        .collect())
}

pub fn fix_text(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut fixed_text = String::from(text);

    // process unique words in text. words do not contain punctuations
    let words_iter = text
        .split_whitespace()
        .map(|s| s.trim_matches(|c| !char::is_alphanumeric(c)))
        .filter(|w| contains_czs(&w))
        .unique();

    for word in words_iter {
        let similar_words = get_similar_words(&word)?;

        if similar_words.len() != 1 {
            continue;
        }

        if let Some(correct_word) = similar_words.get(0) {
            let correct_word = fix_case(correct_word, &word);
            fixed_text = fixed_text.replace(word, &correct_word);
        }
    }

    Ok(fixed_text)
}

pub fn save_word(word: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !contains_czs_plus_with_caron(word) {
        return Err("word does not use letters c, z, s or their equivalents with caron".into());
    }

    let word = word.to_lowercase();

    let new_word = NewWord {
        word: word.to_string(),
        sicnik: convert_to_czs(&word),
    };

    let connection = establish_connection();
    let res = diesel::insert_into(words::table)
        .values(&new_word)
        .execute(&connection);

    match res {
        Ok(_) | Err(DatabaseError(UniqueViolation, _)) => {}
        Err(e) => {
            return Err(e.into());
        }
    }

    Ok(())
}

pub fn confirm_text(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let words_iter = text
        .split_whitespace()
        .map(|s| s.trim_matches(|c| !char::is_alphanumeric(c)).to_lowercase())
        .filter(|s| contains_czs_plus_with_caron(s))
        .unique();

    for word in words_iter {
        save_word(&word)?;
    }

    Ok(())
}
