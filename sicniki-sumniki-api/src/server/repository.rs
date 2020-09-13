use mysql::{from_value_opt, Pool, Value};
use mysql::prelude::{Queryable, ToValue};

use crate::convert_to_czs;

pub struct Repository {
    pool: Pool
}

impl Repository {
    pub fn new(connection_string: &str) -> Repository {
        Repository {
            pool: Pool::new(connection_string).unwrap()
        }
    }

    pub fn get_similar_words(&self, word: &str) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
        let sicnik = convert_to_czs(word);

        let mut conn = self.pool.get_conn()?;
        let rows: Vec<(Value,)> = conn.exec("SELECT word FROM words WHERE sicnik = :sicnik", (sicnik,))?;

        let mut similar_words = vec![];

        for row in rows.iter() {
            if let Ok(word) = from_value_opt::<String>(row.0.to_value()) {
                similar_words.push(word);
            }
        }

        Ok(similar_words)
    }

    pub fn insert_new_word(&self, word: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let sicnik = convert_to_czs(word);

        let mut conn = self.pool.get_conn()?;
        conn.exec_drop("INSERT IGNORE INTO words (word, sicnik) VALUES (:word, :sicnik)", (word.to_lowercase(), sicnik))?;

        Ok(())
    }
}