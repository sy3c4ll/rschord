use crate::note::Duration;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Subdivision(usize);

impl Subdivision {
    pub const WHOLE_NOTE: Self = Subdivision::new(0);
    pub const HALF_NOTE: Self = Subdivision::new(1);
    pub const QUARTER_NOTE: Self = Subdivision::new(2);
    pub const X8TH_NOTE: Self = Subdivision::new(3);
    pub const X16TH_NOTE: Self = Subdivision::new(4);
    pub const X32ND_NOTE: Self = Subdivision::new(5);
    pub const X64TH_NOTE: Self = Subdivision::new(6);
    pub const X128TH_NOTE: Self = Subdivision::new(7);
    pub const SEMIBREVE: Self = Subdivision::new(0);
    pub const MINIM: Self = Subdivision::new(1);
    pub const CROTCHET: Self = Subdivision::new(2);
    pub const QUAVER: Self = Subdivision::new(3);
    pub const SEMIQUAVER: Self = Subdivision::new(4);
    pub const DEMISEMIQUAVER: Self = Subdivision::new(5);
    pub const HEMIDEMISEMIQUAVER: Self = Subdivision::new(6);
    pub const SEMIHEMIDEMISEMIQUAVER: Self = Subdivision::new(7);
    pub const QUASIHEMIDEMISEMIQUAVER: Self = Subdivision::new(7);
    pub const fn new(exponent: usize) -> Self {
        Subdivision(exponent)
    }
    pub const fn name(&self) -> &'static str {
        match *self {
            Self::SEMIBREVE => "\u{1d15d}",
            Self::MINIM => "\u{1d15e}",
            Self::CROTCHET => "\u{1d15f}",
            Self::QUAVER => "\u{1d160}",
            Self::SEMIQUAVER => "\u{1d161}",
            Self::DEMISEMIQUAVER => "\u{1d162}",
            Self::HEMIDEMISEMIQUAVER => "\u{1d163}",
            Self::SEMIHEMIDEMISEMIQUAVER => "\u{1d164}",
            _ => "\u{25a1}",
        }
    }
    pub const fn exponent(&self) -> usize {
        self.0
    }
    pub const fn nth(&self) -> usize {
        1 << self.0
    }
    pub const fn to_duration(self) -> Duration {
        Duration {
            length: 1,
            unit: self,
        }
    }
}

impl fmt::Display for Subdivision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Subdivision {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "\u{1d15d}" => Ok(Self::SEMIBREVE),
            "\u{1d15e}" => Ok(Self::MINIM),
            "\u{1d15f}" => Ok(Self::CROTCHET),
            "\u{1d160}" => Ok(Self::QUAVER),
            "\u{1d161}" => Ok(Self::SEMIQUAVER),
            "\u{1d162}" => Ok(Self::DEMISEMIQUAVER),
            "\u{1d163}" => Ok(Self::HEMIDEMISEMIQUAVER),
            "\u{1d164}" => Ok(Self::SEMIHEMIDEMISEMIQUAVER),
            _ => Err(()),
        }
    }
}

impl From<Subdivision> for usize {
    fn from(subdivision: Subdivision) -> Self {
        subdivision.exponent()
    }
}

impl From<usize> for Subdivision {
    fn from(exponent: usize) -> Self {
        Subdivision::new(exponent)
    }
}

impl PartialOrd for Subdivision {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Subdivision {
    fn cmp(&self, other: &Self) -> Ordering {
        other.exponent().cmp(&self.exponent())
    }
}
