
/*
 * The primary result we are looking for
 */
extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;


#[derive(Clone)]
pub struct ThesaurusEntry {
    word: String,
    part_of_speech: String,
    // The meaning of the term
    meaning: String,
    synonyms: Vec<String>,
    antonyms: Option<Vec<String>>,
}

pub trait DefinitionSource {
    fn fetch_result(&self, word: String) -> Result<ThesaurusEntry>;
}


pub struct DefaultThesaurus {
    host: String,
}

impl DefaultThesaurus {
    pub fn new() -> DefaultThesaurus {
        DefaultThesaurus {
            host: "thesaurus.com"
        }
    }
}

pub trait ThesaurusSource {
    fn retrieve_entries(&self, word: String) -> Result<ThesaurusEntry>;
}

impl ThesaurusSource for DefaultThesaurus {
    //! Fetch the page where the entry is stored
    fn fetch_page(&self, word: String) -> Result<Response> {
        let response = client.get(&self.host + "/browse" + &word).send();

        match response {
            Result::Ok(response) => response,
            Result::Err(err) =>
              panic!("Unwrapping with invalid URL, the following error occurred: {:?}", err),
        }
    }
}
