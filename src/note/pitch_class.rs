use crate::note::{Accidental, Chromatic, Diatonic};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PitchClass {
    pub base: Diatonic,
    pub accidental: Accidental,
}

impl PitchClass {
    pub const fn new(base: Diatonic, accidental: Accidental) -> Self {
        PitchClass { base, accidental }
    }
    pub const fn chromatic(self) -> Chromatic {
        self.base.with(self.accidental)
    }
    pub fn enharmonic(&self, rhs: &Self) -> bool {
        self.chromatic() == rhs.chromatic()
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.base, self.accidental)
    }
}

impl FromStr for PitchClass {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        let (base, accidental) = s.split_at(1);
        let (base, accidental) = (base.parse()?, accidental.parse()?);
        Ok(PitchClass { base, accidental })
    }
}

impl From<PitchClass> for Chromatic {
    fn from(pitch_class: PitchClass) -> Self {
        pitch_class.chromatic()
    }
}

impl From<PitchClass> for (Diatonic, Accidental) {
    fn from(PitchClass { base, accidental }: PitchClass) -> Self {
        (base, accidental)
    }
}

impl From<(Diatonic, Accidental)> for PitchClass {
    fn from((base, accidental): (Diatonic, Accidental)) -> Self {
        PitchClass { base, accidental }
    }
}
