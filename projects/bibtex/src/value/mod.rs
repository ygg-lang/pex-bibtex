use crate::Bibliography;
use indexmap::IndexMap;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Bibtex {
    comments: Vec<String>,
    preambles: Vec<String>,
    const_map: IndexMap<&'static str, &'static str>,
    variables: IndexMap<String, String>,
    bibliographies: Vec<Bibliography>,
}
