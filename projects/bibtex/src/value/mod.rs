use indexmap::IndexMap;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
mod der;
mod parser;
mod ser;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Bibtex {
    comments: Vec<String>,
    preambles: Vec<String>,
    const_map: IndexMap<&'static str, &'static str>,
    variables: IndexMap<String, String>,
    bibliographies: Vec<Bibliography>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bibliography {
    entry_type: String,
    citation_key: String,
    tags: IndexMap<String, String>,
}
