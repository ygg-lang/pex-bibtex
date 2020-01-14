use crate::BibTeX;
use pex::StopBecause;
use std::{io::ErrorKind, path::Path, str::FromStr};

impl FromStr for BibTeX {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        todo!()
    }
}

impl BibTeX {
    /// Load a bibliography from a file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bibtex::BibTeX;
    /// # fn main() -> std::io::Result<()> {
    /// BibTeX::from_path("tex-book/refs.bib")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_path<P>(path: P) -> std::io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let s = std::fs::read_to_string(path)?;
        match s.parse() {
            Ok(bib) => Ok(bib),
            Err(e) => Err(std::io::Error::new(ErrorKind::InvalidData, e)),
        }
    }
}
