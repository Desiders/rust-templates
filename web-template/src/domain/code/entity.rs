use serde::Serialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Serialize)]
pub struct Code {
    pub code: u16,
    pub name: &'static str,
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}
