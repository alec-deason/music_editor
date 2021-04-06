use ir::{Note, Duration, Pitch, Octave, Chord, Sounding, Line};

pub trait ToSmufl {
    fn to_smufl(&self) -> String;
}

fn vertical_offset(amount: i32) -> Option<char> {
    match amount {
        -8 => Some(''),
        -7 => Some(''),
        -6 => Some(''),
        -5 => Some(''),
        -4 => Some(''),
        -3 => Some(''),
        -2 => Some(''),
        -1 => Some(''),
        0 => None,
        1 => Some(''),
        2 => Some(''),
        3 => Some(''),
        4 => Some(''),
        5 => Some(''),
        6 => Some(''),
        7 => Some(''),
        8 => Some(''),
        _ => todo!("{}", amount),
    }
}

impl ToSmufl for Line {
    fn to_smufl(&self) -> String {
        let mut result = String::new();
        result.push(''); // barline
        let mut beat = 0.0;
        for sounding in &self.soundings {
            match sounding {
                Sounding::Note(n) => {
                    beat += n.duration.as_f32() * 4.0;
                    result.push(''); // stave
                    result.push(' '); // space
                    result.push(' '); // space
                    let offset = n.octave.0 as i32 * 12 + n.pitch.0 - 4*12 - 1;
                    if offset < -4 {
                        let added_staves = offset.abs() - 4;
                        let stave = match added_staves {
                            1 => '',
                            2 => '',
                            3 => '',
                            4 => '',
                            _ => todo!(),
                        };
                        if let Some(c) = vertical_offset(offset-1) {
                            result.push(c);
                        }
                        result.push(stave);
                    }
                    if let Some(c) = vertical_offset(offset) {
                        result.push(c);
                    }
                    let note = match n.duration {
                        Duration::WHOLE => '',
                        Duration::HALF => {
                            if offset >= 0 {
                                ''
                            } else {
                                ''
                            }
                        },
                        Duration::QUARTER => {
                            if offset >= 0 {
                                ''
                            } else {
                                ''
                            }
                        },
                        Duration::EIGHTH => {
                            if offset >= 0 {
                                ''
                            } else {
                                ''
                            }
                        },
                        Duration::SIXTEENTH => {
                            if offset >= 0 {
                                ''
                            } else {
                                ''
                            }
                        },
                        _ => todo!(),
                    };
                    result.push(note);
                    if n.duration == Duration::EIGHTH {
                        //result.push(''); // stave
                    }
                    result.push(' '); // space
                    result.push(' '); // space
                    if beat == 4.0 {
                        beat = 0.0;
                        result.push(''); // stave
                        result.push(' '); // space
                        result.push(' '); // space
                        result.push(''); // barline
                    }
                },
                _ => unimplemented!(),
            }
        }
        result
    }
}
