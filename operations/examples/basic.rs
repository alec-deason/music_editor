use operations::{Score, Note, Beat, Pitch, Selection, Context, Location, Selections, PrependNote, Operation, Octave, PitchName};

fn main() {
    let mut ctx = Context {
        score: Score::default(),
        selections: Selections(vec![Selection { begin: Location(Beat(0)), end: Location(Beat(0)) }])
    };

    let operation = PrependNote {
        note: Note {
            pitch: Pitch {
                class: PitchName::A,
                ..Default::default()
            },
            octave: Octave(4)
        },
        duration: Beat(4),
        selections: None,
    };

    operation.apply(&mut ctx);

    let operation = PrependNote {
        note: Note {
            pitch: Pitch {
                class: PitchName::B,
                ..Default::default()
            },
            octave: Octave(4)
        },
        duration: Beat(4),
        selections: None,
    };

    operation.apply(&mut ctx);


    let mei = ctx.score.to_mei();
    std::fs::write("/tmp/test.svg", &mei.to_svg());
}
