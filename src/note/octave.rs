use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Octave(isize);

impl Octave {
    pub const fn new(octave: isize) -> Self {
        Octave(octave)
    }
    pub const fn numeric(&self) -> isize {
        self.0
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.numeric().fmt(f)
    }
}

impl FromStr for Octave {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        isize::from_str(s).map(Octave::new).map_err(|_| ())
    }
}

impl From<isize> for Octave {
    fn from(octave: isize) -> Self {
        Octave::new(octave)
    }
}

impl From<Octave> for isize {
    fn from(octave: Octave) -> Self {
        octave.numeric()
    }
}
