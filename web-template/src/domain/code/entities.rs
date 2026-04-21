use serde::Serialize;
use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Serialize)]
pub struct Code {
    #[allow(clippy::struct_field_names)]
    pub code: u16,
    pub name: &'static str,
    pub message: Cow<'static, str>,
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}
