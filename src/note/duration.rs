use crate::note::Subdivision;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq)]
pub struct Duration {
    pub length: usize,
    pub unit: Subdivision,
}

impl Duration {
    pub const fn new(length: usize, unit: Subdivision) -> Self {
        Duration { length, unit }
    }
}

impl From<Duration> for (usize, Subdivision) {
    fn from(Duration { length, unit }: Duration) -> Self {
        (length, unit)
    }
}

impl From<(usize, Subdivision)> for Duration {
    fn from((length, unit): (usize, Subdivision)) -> Self {
        Duration { length, unit }
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.unit.exponent().cmp(&other.unit.exponent()) {
            Ordering::Less => {
                (self.length << (other.unit.exponent() - self.unit.exponent())).cmp(&other.length)
            }
            Ordering::Equal => self.length.cmp(&other.length),
            Ordering::Greater => self
                .length
                .cmp(&(other.length << (self.unit.exponent() - other.unit.exponent()))),
        }
    }
}
