use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Ignore {
    // solidhunter-disable-next-line
    NextLine,
    // solidhunter-disable-line
    Line,
}

impl ToString for Ignore {
    fn to_string(&self) -> String {
        match self {
            Ignore::NextLine => "solidhunter-disable-next-line",
            Ignore::Line => "solidhunter-disable-line",
        }
        .to_string()
    }
}

impl Ignore {
    pub fn iter() -> impl Iterator<Item = Ignore> {
        [Ignore::NextLine, Ignore::Line].iter().copied()
    }
}
