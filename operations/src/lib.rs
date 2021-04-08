#[derive(Clone, Default, Debug)]
pub struct Score {
    events: Vec<Event>,
}

impl Score {
    pub fn to_mei(&self) -> ir::Mei {
        let mut mei = ir::Mei::default();
        mei.mei_head = Some(ir::MeiHead {
            ..Default::default()
        });
        let mut section = ir::Section::default();

        let mut layer = ir::Layer::default();
        layer.n = Some(1);
        let mut beat = Beat::default();
        for event in &self.events {
            if event.start > beat {
                let rest = event.start - beat;
                let dur = (1.0 / (rest.0 as f32 / 16.0*4.0)).floor() as u32;
                layer.events.push(ir::EventLike::Rest(ir::Rest { dur: Some(dur) }));
            }
            let dur = (1.0 / (event.duration.0 as f32 / 16.0*4.0)).floor() as u32;
            layer.events.push(ir::EventLike::Note(ir::Note {
                pname: Some(event.note.pitch.class.to_string()),
                oct: event.note.octave.0,
                dur: Some(dur),
                ..Default::default()
            }));
            beat = event.start + event.duration;
            if beat.0 % 4 == 0 {
                let n = layer.n;
                let measure = ir::Measure {
                    n: Some(1),
                    staves: vec![ir::Staff {
                        layers: vec![layer],
                        ..Default::default()
                    }]
                };
                section.measures.push(measure);
                layer = ir::Layer::default();
                layer.n = n.map(|n| n+1);
            }
        }

        mei.music = Some(ir::Music {
            body: Some(ir::Body {
                mdivs: vec![
                    ir::MDiv {
                        score: Some(ir::Score {
                            score_def: Some(ir::ScoreDef {
                                meter_count: Some(4),
                                meter_unit: Some(4),
                                key_sig: Some("0".to_string()),
                                key_mode: Some("major".to_string()),
                                staff_grp: Some(ir::StaffGrp {
                                    staff_def: Some(ir::StaffDef {
                                        n: Some(1),
                                        clef_line: Some(2),
                                        clef_shape: Some("G".to_string()),
                                        clef_dis_place: Some("below".to_string()),
                                        lines: Some(5),
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            sections: vec![section],
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    ..Default::default()
            }),
            ..Default::default()
        });

         mei
    }
}

#[derive(Clone, Debug)]
pub struct Event {
    note: Note,
    duration: Beat,
    start: Beat,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Beat(pub u32);


impl std::ops::Add for Beat {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Beat(self.0+other.0)
    }
}
impl std::ops::AddAssign for Beat {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl std::ops::Sub for Beat {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Beat(self.0-other.0)
    }
}
impl std::ops::SubAssign for Beat {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Note {
    pub pitch: Pitch,
    pub octave: Octave,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            pitch: Default::default(),
            octave: Octave(4)
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Pitch {
    pub class: PitchName,
    pub accidental: Accidental
}

impl Default for Pitch {
    fn default() -> Self {
        Self {
            class: PitchName::A,
            accidental: Accidental::Natural
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PitchName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl std::fmt::Display for PitchName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PitchName::A => "a",
            PitchName::B => "b",
            PitchName::C => "c",
            PitchName::D => "d",
            PitchName::E => "e",
            PitchName::F => "f",
            PitchName::G => "g",
        };
        write!(f, "{}", name)
    }
}



#[derive(Copy, Clone, Debug)]
pub enum Accidental {
    Sharp,
    Flat,
    Natural
}

impl std::fmt::Display for Accidental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Accidental::Sharp => "s",
            Accidental::Flat => "f",
            Accidental::Natural => "n",
        };
        write!(f, "{}", name)
    }
}


#[derive(Copy, Clone, Default, Debug)]
pub struct Octave(pub u32);

#[derive(Clone, Default, Debug)]
pub struct Selection {
    pub begin: Location,
    pub end: Location,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Location(pub Beat);

pub trait Operation {
    fn apply(&self, ctx: &mut Context);
}

pub struct AppendNote {
    pub note: Note,
    pub duration: Beat,
    pub selections: Option<Vec<u32>>,
}

impl Operation for AppendNote {
    fn apply(&self, ctx: &mut Context) {
        let selections = if let Some(selections) = self.selections.clone() {
            selections
        } else {
            (0..ctx.selections.0.len() as u32).collect()
        };
        for selection_id in selections {
            let selection:&Selection = &ctx.selections.0[selection_id as usize];
            let mut location = selection.end;
            ctx.insert_event_at_location(location, Event {
                note: self.note,
                start: location.0,
                duration: self.duration
            });
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Selections(pub Vec<Selection>);

#[derive(Clone, Default, Debug)]
pub struct Context {
    pub score: Score,
    pub selections: Selections,
}

impl Context {
    fn insert_event_at_location(&mut self, location: Location, event: Event) {
        let mut target_idx = 0;
        for event in &self.score.events {
            target_idx += 1;
            if event.start > location.0 {
                break
            }
        }
        let insertion_beat = location.0;
        for event in &mut self.score.events {
            if event.start >= insertion_beat {
                event.start += event.duration;
            }
        }

        for selection in &mut self.selections.0 {
            if selection.begin.0 > insertion_beat {
                selection.begin.0 += event.duration;
            }
            if selection.end.0 >= insertion_beat {
                selection.end.0 += event.duration;
            }
        }

        self.score.events.insert(target_idx, event);
    }
}
