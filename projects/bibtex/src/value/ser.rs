use crate::value::Bibliography;
use std::fmt::{Display, Formatter};

impl Display for Bibliography {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}{{{}}},\n", self.entry_type, self.citation_key)?;
        for (key, value) in &self.tags {
            write!(f, "    {} = {{{}}},\n", key, value)?;
        }
        write!(f, "}}")
    }
}
