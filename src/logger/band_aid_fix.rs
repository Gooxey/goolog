use core::{
    fmt,
    fmt::{
        Display,
        Write,
    },
    iter::Take,
    str::Chars,
};

pub(super) struct DisplayChars<'a> {
    chars: Option<Take<Chars<'a>>>,
    missing: usize,
}
impl<'a> DisplayChars<'a> {
    pub(super) fn new(chars: Option<Take<Chars<'a>>>, missing: usize) -> Self {
        Self { chars, missing }
    }
}
impl<'a> From<&'a str> for DisplayChars<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            chars: value.chars().take(usize::MAX).into(),
            missing: 0,
        }
    }
}
impl<'a> From<Take<Chars<'a>>> for DisplayChars<'a> {
    fn from(value: Take<Chars<'a>>) -> Self {
        Self {
            chars: value.into(),
            missing: 0,
        }
    }
}
impl Display for DisplayChars<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(chars) = &self.chars {
            for char in chars.clone() {
                f.write_char(char)?
            }
        }

        for _ in 0..self.missing {
            f.write_char(' ')?
        }

        Ok(())
    }
}
