use crate::note::{Accidental, Diatonic, MidiNote, Octave, PitchClass, Tone};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pitch {
    pub pitch_class: PitchClass,
    pub octave: Octave,
}

impl Pitch {
    pub const MIDDLE_C: Self = Pitch::new(
        PitchClass::new(Diatonic::C, Accidental::NATURAL),
        Octave::new(4),
    );
    pub const STUTTGART: Self = Pitch::new(
        PitchClass::new(Diatonic::A, Accidental::NATURAL),
        Octave::new(4),
    );
    pub const MIDI_MIN: Self = Pitch::new(
        PitchClass::new(Diatonic::C, Accidental::NATURAL),
        Octave::new(-1),
    );
    pub const MIDI_MAX: Self = Pitch::new(
        PitchClass::new(Diatonic::G, Accidental::NATURAL),
        Octave::new(9),
    );
    pub const PIANO_MIN: Self = Pitch::new(
        PitchClass::new(Diatonic::A, Accidental::NATURAL),
        Octave::new(0),
    );
    pub const PIANO_MAX: Self = Pitch::new(
        PitchClass::new(Diatonic::C, Accidental::NATURAL),
        Octave::new(8),
    );
    pub const fn new(pitch_class: PitchClass, octave: Octave) -> Self {
        Pitch {
            pitch_class,
            octave,
        }
    }
    pub const fn to_tone(self) -> Tone {
        self.to_midi_note().to_tone()
    }
    pub const fn to_midi_note(self) -> MidiNote {
        MidiNote::from_pitch(self)
    }
    pub const fn enharmonic(&self, other: &Self) -> bool {
        self.to_midi_note().as_i8() == other.to_midi_note().as_i8()
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.pitch_class, self.octave)
    }
}

impl FromStr for Pitch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pitch_class, octave) = s
            .split_once(|c: char| c.is_ascii_digit() || c == '-')
            .ok_or(())?;
        let (pitch_class, octave) = (pitch_class.parse()?, octave.parse()?);
        Ok(Pitch {
            pitch_class,
            octave,
        })
    }
}

impl From<Pitch> for Tone {
    fn from(pitch: Pitch) -> Self {
        pitch.to_tone()
    }
}

impl From<Pitch> for (PitchClass, Octave) {
    fn from(
        Pitch {
            pitch_class,
            octave,
        }: Pitch,
    ) -> Self {
        (pitch_class, octave)
    }
}

impl From<(PitchClass, Octave)> for Pitch {
    fn from((pitch_class, octave): (PitchClass, Octave)) -> Self {
        Pitch {
            pitch_class,
            octave,
        }
    }
}

impl PartialOrd for Pitch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pitch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_midi_note().cmp(&other.to_midi_note())
    }
}
