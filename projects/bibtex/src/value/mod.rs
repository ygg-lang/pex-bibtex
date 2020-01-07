use std::collections::BTreeMap;

mod der;
mod ser;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Bibtex {
    comments: Vec<String>,
    preambles: Vec<String>,
    const_map: BTreeMap<&'static str, &'static str>,
    variables: BTreeMap<String, String>,
    bibliographies: Vec<Bibliography>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bibliography {
    entry_type: String,
    citation_key: String,
    tags: BTreeMap<String, String>,
}
