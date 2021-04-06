use ir::{Note, Duration, Pitch, Octave, Chord, Sounding, Line};
use ir_to_smufl::ToSmufl;

fn main() {
    let line = Line {
        soundings: vec![
            Sounding::Note(Note {
                duration: Duration::QUARTER,
                pitch: Pitch(0),
                octave: Octave(4),
            }),
            Sounding::Note(Note {
                duration: Duration::QUARTER,
                pitch: Pitch(3),
                octave: Octave(4),
            }),
            Sounding::Note(Note {
                duration: Duration::HALF,
                pitch: Pitch(6),
                octave: Octave(3),
            }),
            Sounding::Note(Note {
                duration: Duration::QUARTER,
                pitch: Pitch(6),
                octave: Octave(3),
            }),
            Sounding::Note(Note {
                duration: Duration::QUARTER,
                pitch: Pitch(2),
                octave: Octave(4),
            }),
            Sounding::Note(Note {
                duration: Duration::EIGHTH,
                pitch: Pitch(1),
                octave: Octave(4),
            }),
            Sounding::Note(Note {
                duration: Duration::EIGHTH,
                pitch: Pitch(0),
                octave: Octave(4),
            }),
            Sounding::Note(Note {
                duration: Duration::EIGHTH,
                pitch: Pitch(0),
                octave: Octave(4),
            }),
            Sounding::Note(Note {
                duration: Duration::EIGHTH,
                pitch: Pitch(0),
                octave: Octave(4),
            }),
            Sounding::Note(Note {
                duration: Duration::EIGHTH,
                pitch: Pitch(0),
                octave: Octave(4),
            }),
        ]
    };
    println!("{}", line.to_smufl());
}
