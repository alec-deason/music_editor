pub mod front;

pub struct Note {
    pub duration: Duration,
    pub pitch: Pitch,
    pub octave: Octave,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Duration {
    pub numerator: u8,
    pub denominator: u8,
}

impl Duration {
    pub const WHOLE:Duration = Duration { numerator: 1, denominator: 1 };
    pub const HALF:Duration = Duration { numerator: 1, denominator: 2 };
    pub const QUARTER:Duration = Duration { numerator: 1, denominator: 4 };
    pub const EIGHTH:Duration = Duration { numerator: 1, denominator: 8 };
    pub const SIXTEENTH:Duration = Duration { numerator: 1, denominator: 16 };

    pub fn as_f32(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }
}

pub struct Pitch(pub i32);
pub struct Octave(pub u8);

pub struct Chord(pub Vec<Note>);

pub enum Sounding {
    Note(Note),
    Chord(Chord),
}

pub struct Line {
    pub soundings: Vec<Sounding>,
}
