use crate::note::{Duration, Pitch};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Note {
    pub pitch: Pitch,
    pub value: Duration,
}

impl Note {
    pub const fn new(pitch: Pitch, value: Duration) -> Self {
        Note { pitch, value }
    }
}

impl From<Note> for (Pitch, Duration) {
    fn from(Note { pitch, value }: Note) -> Self {
        (pitch, value)
    }
}

impl From<(Pitch, Duration)> for Note {
    fn from((pitch, value): (Pitch, Duration)) -> Self {
        Note { pitch, value }
    }
}
