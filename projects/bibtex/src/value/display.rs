use super::*;

impl Display for BibTeX {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for comment in &self.comments {
            writeln!(f, "{}", comment)?;
        }
        for preamble in &self.preambles {
            writeln!(f, "{}", preamble)?;
        }
        for (key, value) in &self.const_map {
            writeln!(f, "@string{{ {} = {} }}", key, value)?;
        }
        for (key, value) in &self.variables {
            writeln!(f, "@preamble{{ {} = {} }}", key, value)?;
        }
        for bibliography in &self.bibliographies {
            writeln!(f, "{}", bibliography)?;
        }
        Ok(())
    }
}
