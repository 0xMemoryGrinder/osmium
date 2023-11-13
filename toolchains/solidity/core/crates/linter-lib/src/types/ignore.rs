use serde::{Deserialize, Serialize};

macro_rules! define_ignore_enum {
    ($name:ident, $($variant:ident => $str:expr),* $(,)?) => {
        #[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
        pub enum $name {
            $($variant),*
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                match self {
                    $(Self::$variant => $str),*
                }
                .to_string()
            }
        }

        impl $name {
            pub fn iter() -> impl Iterator<Item = Self> {
                [$(Self::$variant),*].iter().copied()
            }
        }
    };
}

define_ignore_enum! {
    Ignore,
    NextLine => "solidhunter-disable-next-line",
    SameLine => "solidhunter-disable-line",
    Disable => "solidhunter-disable",
    Enable => "solidhunter-enable",
}
