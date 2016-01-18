
/*
 * The primary result we are looking for
 */
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
    fn fetch_result(&self, word: String) -> Result<ThesaurusEntry>
}
