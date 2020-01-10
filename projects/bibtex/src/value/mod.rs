use indexmap::IndexMap;
use std::{
    fmt::{Display, Formatter, Write},
    str::FromStr,
};
mod der;
mod display;
mod parser;
#[cfg(feature = "serde")]
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

impl Bibliography {
    pub fn authors(&self) -> Vec<&str> {
        match self.tags.get("author") {
            Some(s) => s.split("and").map(|s| s.trim()).collect(),
            None => {
                vec![]
            }
        }
    }
}
