use std::borrow::Cow;

use super::entities::Code;

pub const UNEXPECTED: Code = Code {
    code: 1000,
    name: "Unexpected",
    message: Cow::Borrowed("Unexpected error occurred"),
};
