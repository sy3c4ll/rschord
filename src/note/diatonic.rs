use crate::note::{Accidental, Chromatic};
use std::fmt;
use std::ops::{Add, Shl, Shr};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Diatonic {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl Diatonic {
    pub const NOTES: u8 = 7;
    pub const fn name(&self) -> &'static str {
        match self {
            Diatonic::C => "C",
            Diatonic::D => "D",
            Diatonic::E => "E",
            Diatonic::F => "F",
            Diatonic::G => "G",
            Diatonic::A => "A",
            Diatonic::B => "B",
        }
    }
    pub const fn chromatic(self) -> Chromatic {
        Chromatic::wrapping_chr(self.ord())
    }
    pub const fn ord(self) -> isize {
        self as isize
    }
    pub const fn checked_chr(ord: isize) -> Option<Self> {
        match ord {
            0 => Some(Diatonic::C),
            2 => Some(Diatonic::D),
            4 => Some(Diatonic::E),
            5 => Some(Diatonic::F),
            7 => Some(Diatonic::G),
            9 => Some(Diatonic::A),
            11 => Some(Diatonic::B),
            _ => None
        }
    }
    pub const fn flatten(self, semitones: isize) -> Chromatic {
        self.chromatic().flatten(semitones)
    }
    pub const fn sharpen(self, semitones: isize) -> Chromatic {
        self.chromatic().sharpen(semitones)
    }
    pub const fn with(self, accidental: Accidental) -> Chromatic {
        self.chromatic().with(accidental)
    }
}

impl fmt::Display for Diatonic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Diatonic {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" | "c" => Ok(Diatonic::C),
            "D" | "d" => Ok(Diatonic::D),
            "E" | "e" => Ok(Diatonic::E),
            "F" | "f" => Ok(Diatonic::F),
            "G" | "g" => Ok(Diatonic::G),
            "A" | "a" => Ok(Diatonic::A),
            "B" | "b" => Ok(Diatonic::B),
            _ => Err(()),
        }
    }
}

impl From<Diatonic> for Chromatic {
    fn from(note: Diatonic) -> Self {
        note.chromatic()
    }
}

impl From<Diatonic> for isize {
    fn from(note: Diatonic) -> Self {
        note.ord()
    }
}

impl TryFrom<isize> for Diatonic {
    type Error = ();
    fn try_from(note: isize) -> Result<Self, Self::Error> {
        Diatonic::checked_chr(note).ok_or(())
    }
}

impl Shl<isize> for Diatonic {
    type Output = Chromatic;
    fn shl(self, rhs: isize) -> Self::Output {
        self.flatten(rhs)
    }
}

impl Shr<isize> for Diatonic {
    type Output = Chromatic;
    fn shr(self, rhs: isize) -> Self::Output {
        self.sharpen(rhs)
    }
}

impl Add<Accidental> for Diatonic {
    type Output = Chromatic;
    fn add(self, rhs: Accidental) -> Self::Output {
        self.with(rhs)
    }
}
