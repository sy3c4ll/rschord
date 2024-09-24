use crate::note::{Accidental, Diatonic, PitchClass};
use std::fmt;
use std::ops::{Add, AddAssign, Shl, ShlAssign, Shr, ShrAssign};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Chromatic {
    C = 0,
    CsDb = 1,
    D = 2,
    DsEb = 3,
    E = 4,
    F = 5,
    FsGb = 6,
    G = 7,
    GsAb = 8,
    A = 9,
    AsBb = 10,
    B = 11,
}

impl Chromatic {
    pub const NOTES: u8 = 12;
    pub const fn name(&self) -> &'static str {
        match self {
            Chromatic::C => "C",
            Chromatic::CsDb => "C♯/D♭",
            Chromatic::D => "D",
            Chromatic::DsEb => "D♯/E♭",
            Chromatic::E => "E",
            Chromatic::F => "F",
            Chromatic::FsGb => "F♯/G♭",
            Chromatic::G => "G",
            Chromatic::GsAb => "G♯/A♭",
            Chromatic::A => "A",
            Chromatic::AsBb => "A♯/B♭",
            Chromatic::B => "B",
        }
    }
    pub const fn try_diatonic(self) -> Option<Diatonic> {
        Diatonic::checked_chr(self.ord())
    }
    pub const fn ord(self) -> isize {
        self as isize
    }
    pub const fn wrapping_chr(ord: isize) -> Self {
        match ord % Chromatic::NOTES as isize {
            0 => Chromatic::C,
            1 => Chromatic::CsDb,
            2 => Chromatic::D,
            3 => Chromatic::DsEb,
            4 => Chromatic::E,
            5 => Chromatic::F,
            6 => Chromatic::FsGb,
            7 => Chromatic::G,
            8 => Chromatic::GsAb,
            9 => Chromatic::A,
            10 => Chromatic::AsBb,
            11 => Chromatic::B,
            _ => unreachable!(),
        }
    }
    pub const fn checked_chr(ord: isize) -> Option<Self> {
        match ord % Chromatic::NOTES as isize {
            0 => Some(Chromatic::C),
            1 => Some(Chromatic::CsDb),
            2 => Some(Chromatic::D),
            3 => Some(Chromatic::DsEb),
            4 => Some(Chromatic::E),
            5 => Some(Chromatic::F),
            6 => Some(Chromatic::FsGb),
            7 => Some(Chromatic::G),
            8 => Some(Chromatic::GsAb),
            9 => Some(Chromatic::A),
            10 => Some(Chromatic::AsBb),
            11 => Some(Chromatic::B),
            _ => None,
        }
    }
    pub const fn flatten(self, semitones: isize) -> Chromatic {
        Chromatic::wrapping_chr(self.ord() - semitones)
    }
    pub const fn sharpen(self, semitones: isize) -> Chromatic {
        Chromatic::wrapping_chr(self.ord() + semitones)
    }
    pub const fn with(self, accidental: Accidental) -> Chromatic {
        self.sharpen(accidental.pitch_shift())
    }
}

impl fmt::Display for Chromatic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Chromatic {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PitchClass::from_str(s).map(PitchClass::chromatic)
        /* match s {
            "C" => Ok(Chromatic::C),
            "C♯" | "D♭" | "C♯/D♭" => Ok(Chromatic::CsDb),
            "D" => Ok(Chromatic::D),
            "D♯" | "E♭" | "D♯/E♭" => Ok(Chromatic::DsEb),
            "E" => Ok(Chromatic::E),
            "F" => Ok(Chromatic::F),
            "F♯" | "G♭" | "F♯/G♭" => Ok(Chromatic::FsGb),
            "G" => Ok(Chromatic::G),
            "G♯" | "A♭" | "G♯/A♭" => Ok(Chromatic::GsAb),
            "A" => Ok(Chromatic::A),
            "A♯" | "B♭" | "A♯/B♭" => Ok(Chromatic::AsBb),
            "B" => Ok(Chromatic::B),
            _ => Err(()),
        } */
    }
}

impl TryFrom<Chromatic> for Diatonic {
    type Error = ();
    fn try_from(note: Chromatic) -> Result<Self, Self::Error> {
        note.try_diatonic().ok_or(())
    }
}

impl From<Chromatic> for isize {
    fn from(note: Chromatic) -> Self {
        note.ord()
    }
}

impl TryFrom<isize> for Chromatic {
    type Error = ();
    fn try_from(note: isize) -> Result<Self, Self::Error> {
        Chromatic::checked_chr(note).ok_or(())
    }
}

impl Shl<isize> for Chromatic {
    type Output = Chromatic;
    fn shl(self, rhs: isize) -> Self::Output {
        self.flatten(rhs)
    }
}

impl Shr<isize> for Chromatic {
    type Output = Chromatic;
    fn shr(self, rhs: isize) -> Self::Output {
        self.sharpen(rhs)
    }
}

impl Add<Accidental> for Chromatic {
    type Output = Chromatic;
    fn add(self, rhs: Accidental) -> Self::Output {
        self.with(rhs)
    }
}

impl ShlAssign<isize> for Chromatic {
    fn shl_assign(&mut self, rhs: isize) {
        *self = *self << rhs;
    }
}

impl ShrAssign<isize> for Chromatic {
    fn shr_assign(&mut self, rhs: isize) {
        *self = *self >> rhs;
    }
}

impl AddAssign<Accidental> for Chromatic {
    fn add_assign(&mut self, rhs: Accidental) {
        *self = *self + rhs;
    }
}
