use super::*;
use std::fmt::Write;

impl Display for Bibliography {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('@')?;
        f.write_str(&self.entry_type)?;
        f.write_char('{')?;
        f.write_str(&self.citation_key)?;
        if !self.tags.is_empty() {
            f.write_char(',')?;
            f.write_char('\n')?;
        }
        for (key, value) in &self.tags {
            write!(f, "    {} = {{{}}},\n", key, value)?;
        }
        write!(f, "}}")
    }
}
