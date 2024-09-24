use crate::note::{Chromatic, MidiNote, Octave, Pitch};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tone {
    pub chromatic: Chromatic,
    pub octave: Octave,
}

impl Tone {
    pub const MIDDLE_C: Self = Pitch::MIDDLE_C.to_tone();
    pub const STUTTGART: Self = Pitch::STUTTGART.to_tone();
    pub const MIDI_MIN: Self = Pitch::MIDI_MIN.to_tone();
    pub const MIDI_MAX: Self = Pitch::MIDI_MAX.to_tone();
    pub const PIANO_MIN: Self = Pitch::PIANO_MIN.to_tone();
    pub const PIANO_MAX: Self = Pitch::PIANO_MAX.to_tone();
    pub const fn new(chromatic: Chromatic, octave: Octave) -> Self {
        Tone { chromatic, octave }
    }
    pub const fn to_midi_note(self) -> MidiNote {
        MidiNote::from_tone(self)
    }
}

impl fmt::Display for Tone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.chromatic, self.octave)
    }
}

impl FromStr for Tone {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Pitch::from_str(s).map(Pitch::to_tone)
    }
}

impl From<Tone> for (Chromatic, Octave) {
    fn from(Tone { chromatic, octave }: Tone) -> Self {
        (chromatic, octave)
    }
}

impl From<(Chromatic, Octave)> for Tone {
    fn from((chromatic, octave): (Chromatic, Octave)) -> Self {
        Tone { chromatic, octave }
    }
}

impl PartialOrd for Tone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tone {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_midi_note().cmp(&other.to_midi_note())
    }
}
