use ir::{Note, Duration, Pitch, Octave, Chord, Sounding, Line};

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
    let engine = ir_to_svg::EngravingEngine::new("/usr/share/fonts/OTF/BravuraText.otf");
    let tree = engine.to_svg(&line);
    println!("{}", tree.to_string(Default::default()));
}
