use crate::note::{Chromatic, Octave, Pitch, Tone};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MidiNote(i8);

impl MidiNote {
    pub const MIDDLE_C: Self = Pitch::MIDDLE_C.to_midi_note();
    pub const STUTTGART: Self = Pitch::STUTTGART.to_midi_note();
    pub const MIN: Self = MidiNote(0);
    pub const MAX: Self = MidiNote(127);
    pub const fn new(note: i8) -> Self {
        MidiNote(note)
    }
    pub const fn from_pitch(pitch: Pitch) -> Self {
        MidiNote(
            pitch.pitch_class.base.ord() as i8
                + pitch.pitch_class.accidental.pitch_shift() as i8
                + pitch.octave.numeric() as i8 * 12
                + 12,
        )
    }
    pub const fn from_tone(tone: Tone) -> Self {
        MidiNote(tone.chromatic.ord() as i8 + tone.octave.numeric() as i8 * 12 + 12)
    }
    pub const fn to_tone(self) -> Tone {
        Tone {
            chromatic: Chromatic::wrapping_chr(self.0 as isize % 12),
            octave: Octave::new(self.0 as isize / 12 - 1),
        }
    }
    pub const fn as_i8(&self) -> i8 {
        self.0
    }
}

impl fmt::Display for MidiNote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_i8().fmt(f)
    }
}

impl FromStr for MidiNote {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i8::from_str(s).map(MidiNote::new).map_err(|_| ())
    }
}

impl From<Pitch> for MidiNote {
    fn from(pitch: Pitch) -> Self {
        MidiNote::from_pitch(pitch)
    }
}

impl From<Tone> for MidiNote {
    fn from(tone: Tone) -> Self {
        MidiNote::from_tone(tone)
    }
}

impl From<MidiNote> for i8 {
    fn from(note: MidiNote) -> Self {
        note.as_i8()
    }
}

impl From<i8> for MidiNote {
    fn from(note: i8) -> Self {
        MidiNote::new(note)
    }
}
