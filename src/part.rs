use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Part {
    One,
    Two,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letter = match self {
            Part::One => "1",
            Part::Two => "2",
        };
        write!(f, "{}", letter)
    }
}
