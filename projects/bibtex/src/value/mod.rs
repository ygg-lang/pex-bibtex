use crate::Bibliography;
use indexmap::IndexMap;

mod der;
mod parser;
mod ser;

/// Create a book
///
/// # Arguments
///
/// * `citation`:
///
/// returns: Bibliography
///
/// # Examples
///
/// ```
/// use bibtex::BibTeX;
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct BibTeX {
    comments: Vec<String>,
    preambles: Vec<String>,
    const_map: IndexMap<&'static str, &'static str>,
    variables: IndexMap<String, String>,
    bibliographies: Vec<Bibliography>,
}

impl Default for BibTeX {
    fn default() -> Self {
        Self {
            comments: Vec::new(),
            preambles: Vec::new(),
            const_map: IndexMap::new(),
            variables: IndexMap::new(),
            bibliographies: Vec::new(),
        }
    }
}
