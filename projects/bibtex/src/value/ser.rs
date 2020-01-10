use super::*;
use serde::{ser::SerializeMap, Serialize, Serializer};

impl Serialize for Bibliography {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_map(Some(self.tags.len() + 2))?;
        ser.serialize_entry("entry_type", &self.entry_type)?;
        ser.serialize_entry("citation_key", &self.citation_key)?;
        for (key, value) in &self.tags {
            ser.serialize_entry(key, value)?;
        }
        ser.end()
    }
}
