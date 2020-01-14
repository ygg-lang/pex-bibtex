use indexmap::IndexMap;
use std::{
    fmt::{Display, Formatter, Write},
    str::FromStr,
};

#[cfg(feature = "serde")]
mod der;
mod display;
#[cfg(feature = "serde")]
mod parser;
#[cfg(feature = "serde")]
mod ser;

/// A bibliography entry with entry type and citation key
///
/// # Examples
///
/// ```
/// use bibtex::Bibliography;
/// let book = Bibliography::new("book", "texbook").with_tag("year", 1986);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Bibliography {
    entry_type: String,
    citation_key: String,
    tags: IndexMap<String, String>,
}

impl Default for Bibliography {
    fn default() -> Self {
        Self { entry_type: "".to_string(), citation_key: "".to_string(), tags: IndexMap::new() }
    }
}

impl Bibliography {
    /// Create a new bibliography entry with given entry type and citation key
    ///
    /// # Examples
    ///
    /// ```
    /// use bibtex::Bibliography;
    /// let book = Bibliography::new("book", "texbook").with_tag("year", 1986);
    /// ```
    pub fn new<K, V>(entry: K, citation: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        Self { entry_type: entry.to_string(), citation_key: citation.to_string(), tags: Default::default() }
    }
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
    /// use bibtex::Bibliography;
    /// let book = Bibliography::new("book", "texbook").with_tag("year", 1986);
    /// assert_eq!(book.get_entry(), "book");
    /// ```
    pub fn get_entry(&self) -> &str {
        self.entry_type.as_str()
    }
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
    /// use bibtex::Bibliography;
    /// let book = Bibliography::new("book", "texbook").with_tag("year", 1986);
    /// assert_eq!(book.get_citation(), "texbook");
    /// ```
    pub fn get_citation(&self) -> &str {
        self.citation_key.as_str()
    }
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
    /// use bibtex::Bibliography;
    /// let mut book = Bibliography::new("book", "texbook").with_tag("year", 1986);
    /// assert_eq!(book.get_tag("year"), Some("1986"));
    /// ```
    pub fn get_tag(&self, key: &str) -> Option<&str> {
        self.tags.get(key).map(|s| s.as_str())
    }

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
    /// use bibtex::Bibliography;
    /// let mut book = Bibliography::new("book", "texbook").with_tag("year", 1986);
    /// book.set_tag("year", 1980);
    /// assert_eq!(book.get_tag("year"), Some("1980"));
    /// ```
    pub fn set_tag<K, V>(&mut self, key: K, value: V)
    where
        K: ToString,
        V: ToString,
    {
        self.tags.insert(key.to_string(), value.to_string());
    }
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
    /// use bibtex::Bibliography;
    /// let book = Bibliography::new("book", "texbook").with_tag("year", 1986);
    /// assert_eq!(book.get_tag("year"), Some("1986"));
    /// ```
    pub fn with_tag<K, V>(mut self, key: K, value: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        self.set_tag(key, value);
        self
    }
    /// Get authors, return nil if no such field or no author
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
    /// use bibtex::Bibliography;
    /// let book = Bibliography::new("book", "texbook").with_tag("year", 1986);
    /// assert_eq!(book.all_tags().len(), 1);
    /// ```
    pub fn all_tags(&self) -> &IndexMap<String, String> {
        &self.tags
    }
    /// Get authors, return nil if no such field or no author
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
    /// use bibtex::Bibliography;
    /// let mut book = Bibliography::new("book", "texbook").with_tag("year", 1986);
    /// book.mut_tags().clear();
    /// assert_eq!(book.all_tags().len(), 0)
    /// ```
    pub fn mut_tags(&mut self) -> &mut IndexMap<String, String> {
        &mut self.tags
    }

    /// Get authors, return nil if no such field or no author
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
    /// use bibtex::Bibliography;
    /// let mut book =
    ///     Bibliography::new("book", "texbook").with_tag("author", "A. U. Thor and A. N. Other");
    /// assert_eq!(book.authors(), vec!["A. U. Thor", "A. N. Other"]);
    /// ```
    pub fn authors(&self) -> Vec<&str> {
        match self.tags.get("author") {
            Some(s) => s.split("and").map(|s| s.trim()).collect(),
            None => vec![],
        }
    }
}
