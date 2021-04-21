#![feature(btree_retain, map_first_last)]
use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Score {
    events: BTreeSet<Event>,
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
        let mut beat = Pulse::default();
        for event in &self.events {
            if event.start > beat {
                let rest = event.start - beat;
                let dur = (1.0 / (rest.0 as f32 / 16.0*4.0)).floor() as u32;
                layer.events.push(ir::EventLike::Rest(ir::Rest { dur: Some(dur) }));
            }
            let dur = (1.0 / (event.duration.0 as f32 / 16.0*4.0)).floor() as u32;
            let accid = if event.note.pitch.accidental != Accidental::Natural {
                Some(event.note.pitch.accidental.to_string())
            } else {
                None
            };
            layer.events.push(ir::EventLike::Note(ir::Note {
                xml_id: Some(format!("note_{}", event.event_id)),
                pname: Some(event.note.pitch.class.to_string()),
                accid,
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
        if !layer.events.is_empty() {
            let measure = ir::Measure {
                n: Some(1),
                staves: vec![ir::Staff {
                    layers: vec![layer],
                    ..Default::default()
                }]
            };
            section.measures.push(measure);
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

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    event_id: u32,
    note: Note,
    duration: Pulse,
    start: Pulse,
}

impl Event {
    pub fn id(&self) -> u32 {
        self.event_id
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Pulse(pub i32);


impl std::ops::Add for Pulse {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Pulse(self.0+other.0)
    }
}
impl std::ops::AddAssign for Pulse {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl std::ops::Sub for Pulse {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Pulse(self.0-other.0)
    }
}
impl std::ops::SubAssign for Pulse {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl Note {
    pub fn transpose(&mut self, semitones: i32) {
        for _ in 0..semitones.abs() {
            if semitones > 0 {
                *self = self.successor();
            } else {
                *self = self.precurser();
            }
        }
    }

    pub fn semitones(&self) -> u32 {
        self.octave.0 * 12 + self.pitch.semitones()
    }

    pub fn from_semitones(semitones: u32) -> Self {
        let octave = semitones / 12;
        let pitch = Pitch::from_semitones(semitones - octave*12);
        Self {
            pitch,
            octave: Octave(octave)
        }
    }

    pub fn successor(&self) -> Self {
        Self::from_semitones(self.semitones() + 1)
    }

    pub fn precurser(&self) -> Self {
        Self::from_semitones(self.semitones() - 1)
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl Pitch {
    fn semitones(&self) -> u32 {
        let base = self.class.semitones();
        match self.accidental {
            Accidental::Flat => base - 1,
            Accidental::Natural => base,
            Accidental::Sharp => base + 1,
        }
    }

    fn from_semitones(semitones: u32) -> Self {
        match semitones {
            0 => Pitch { class: PitchName::A, accidental: Accidental::Natural },
            1 => Pitch { class: PitchName::A, accidental: Accidental::Sharp },
            2 => Pitch { class: PitchName::B, accidental: Accidental::Natural },
            3 => Pitch { class: PitchName::C, accidental: Accidental::Natural },
            4 => Pitch { class: PitchName::C, accidental: Accidental::Sharp },
            5 => Pitch { class: PitchName::D, accidental: Accidental::Natural },
            6 => Pitch { class: PitchName::D, accidental: Accidental::Sharp },
            7 => Pitch { class: PitchName::E, accidental: Accidental::Natural },
            8 => Pitch { class: PitchName::F, accidental: Accidental::Natural },
            9 => Pitch { class: PitchName::F, accidental: Accidental::Sharp },
            10 => Pitch { class: PitchName::G, accidental: Accidental::Natural },
            11 => Pitch { class: PitchName::G, accidental: Accidental::Sharp },
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PitchName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl PitchName {
    fn semitones(&self) -> u32 {
        match self {
            PitchName::A => 0,
            PitchName::B => 2,
            PitchName::C => 3,
            PitchName::D => 5,
            PitchName::E => 7,
            PitchName::F => 8,
            PitchName::G => 10,
        }
    }
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



#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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


#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Octave(pub u32);

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Selection {
    pub begin: Location,
    pub end: Location,
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub struct Location(pub Pulse);

pub trait Operation {
    fn apply(&self, ctx: &mut Context);
}

pub struct AppendNote {
    pub note: Note,
    pub duration: Pulse,
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
            let selection = &ctx.selections.0[selection_id as usize];
            let mut location = selection.end;
            ctx.insert_event_at_location(location, Event {
                note: self.note,
                start: location.0,
                duration: self.duration,
                ..Default::default()
            });
        }
    }
}

pub enum Duration {
    Pulse(Pulse),
    Event(i32)
}

pub struct MoveSelections {
    pub delta: Duration,
    pub selections: Vec<u32>
}

impl Operation for MoveSelections {
    fn apply(&self, ctx: &mut Context) {
        for selection_id in &self.selections {
            let selection_begin = ctx.selections.0[*selection_id as usize].begin;
            let delta_pulse = match self.delta {
                Duration::Pulse(p) => p,
                Duration::Event(d) => {
                    let mut idx = ctx.score.events.len() as i32 -1;
                    for (i, event) in ctx.score.events.iter().enumerate() {
                        if event.start > selection_begin.0 {
                            idx = i as i32;
                            break
                        }
                    }
                    let initial = ctx.score.events.iter().nth(idx as usize).unwrap().start;
                    idx += d;
                    idx = idx.max(0).min(ctx.score.events.len() as i32 -1);
                    ctx.score.events.iter().nth(idx as usize).unwrap().start - initial
                }
            };
            let selection = &mut ctx.selections.0[*selection_id as usize];
            selection.begin.0 += delta_pulse;
            selection.end.0 += delta_pulse;
        }
    }
}

pub struct MoveSelectionsEnd {
    pub delta: Duration,
    pub selections: Vec<u32>
}

impl Operation for MoveSelectionsEnd {
    fn apply(&self, ctx: &mut Context) {
        for selection_id in &self.selections {
            let selection_end = ctx.selections.0[*selection_id as usize].end;
            let delta_pulse = match self.delta {
                Duration::Pulse(p) => p,
                Duration::Event(d) => {
                    let mut idx = ctx.score.events.len() as i32 -1;
                    for (i, event) in ctx.score.events.iter().enumerate() {
                        if event.start > selection_end.0 {
                            idx = i as i32;
                            break
                        }
                    }
                    let initial = ctx.score.events.iter().nth(idx as usize).unwrap().start;
                    idx += d;
                    idx = idx.max(0).min(ctx.score.events.len() as i32 -1);
                    ctx.score.events.iter().nth(idx as usize).unwrap().start - initial
                }
            };
            let selection = &mut ctx.selections.0[*selection_id as usize];
            selection.end.0 += delta_pulse;
            println!("{:?}", delta_pulse);
            if selection.end.0 < selection.begin.0 {
                std::mem::swap(&mut selection.end, &mut selection.begin);
            }
        }
    }
}

pub struct MoveSelectionsContents {
    pub delta: Duration,
    pub selections: Vec<u32>
}

impl Operation for MoveSelectionsContents {
    fn apply(&self, ctx: &mut Context) {
        for selection_id in &self.selections {
            let selection_begin = ctx.selections.0[*selection_id as usize].begin;
            let delta_pulse = match self.delta {
                Duration::Pulse(p) => p,
                Duration::Event(d) => {
                    let mut idx = ctx.score.events.len() as i32 -1;
                    for (i, event) in ctx.score.events.iter().enumerate() {
                        if event.start > selection_begin.0 {
                            idx = i as i32;
                            break
                        }
                    }
                    let initial = ctx.score.events.iter().nth(idx as usize).unwrap().start;
                    idx += d;
                    idx = idx.max(0).min(ctx.score.events.len() as i32 -1);
                    ctx.score.events.iter().nth(idx as usize).unwrap().start - initial
                }
            };
            let selection = ctx.selections.0[*selection_id as usize].clone();
            let mut new_events = BTreeSet::new();
            while let Some(mut e) = ctx.score.events.pop_first() {
                if e.start >= selection.begin.0 && e.start <= selection.end.0 {
                    e.start += delta_pulse;
                } else {
                    if delta_pulse.0 > 0 {
                        if e.start <= selection.end.0 + delta_pulse {
                            e.start -= delta_pulse;
                        }
                    } else {
                        if e.start >= selection.begin.0 + delta_pulse {
                            e.start -= delta_pulse;
                        }
                    }
                }
                new_events.insert(e);
            }
            ctx.score.events = new_events;

            let selection = &mut ctx.selections.0[*selection_id as usize];
            selection.begin.0 += delta_pulse;
            selection.end.0 += delta_pulse;
        }
    }
}

pub struct TransposeSelectionsContents {
    pub semitones: i32,
    pub selections: Vec<u32>
}

impl Operation for TransposeSelectionsContents {
    fn apply(&self, ctx: &mut Context) {
        for selection_id in &self.selections {
            let selection = ctx.selections.0[*selection_id as usize].clone();
            let mut new_events = BTreeSet::new();
            while let Some(mut e) = ctx.score.events.pop_first() {
                if e.start >= selection.begin.0 && e.start <= selection.end.0 {
                    e.note.transpose(self.semitones);
                }
                new_events.insert(e);
            }
            ctx.score.events = new_events;
        }
    }
}

pub struct DeleteSelections {
    pub selections: Vec<u32>
}

impl Operation for DeleteSelections {
    fn apply(&self, ctx: &mut Context) {
        for selection_id in &self.selections {
            let selection = &ctx.selections.0[*selection_id as usize];
            ctx.score.events.retain(|event| {
                if event.start >= selection.begin.0 && event.start <= selection.end.0 {
                    false
                } else {
                    true
                }
            });
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Selections(pub Vec<Selection>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub score: Score,
    pub selections: Selections,
    next_id: u32,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            score: Score::default(),
            selections: Selections(vec![Selection { begin: Location(Pulse(0)), end: Location(Pulse(0)) }]),
            next_id: 0,
        }
    }
}

impl Context {
    fn insert_event_at_location(&mut self, location: Location, mut event: Event) {
        let mut new_events = BTreeSet::new();
        let insertion_beat = location.0;

        while let Some(mut event) = self.score.events.pop_first() {
            if event.start >= insertion_beat {
                event.start += event.duration;
            }
            new_events.insert(event);
        }
        self.score.events = new_events;

        for selection in &mut self.selections.0 {
            if selection.begin.0 >= insertion_beat {
                selection.begin.0 += event.duration;
            }
            if selection.end.0 >= insertion_beat {
                selection.end.0 += event.duration;
            }
        }

        event.event_id = self.next_id;
        self.next_id += 1;

        self.score.events.insert(event);
    }

    pub fn events_in_selection(&self, selection: usize) -> impl Iterator<Item=&Event> {
        let selection = &self.selections.0[selection];
        let mut iter = self.score.events.iter();
        std::iter::from_fn(move || {
            while let Some(e) = iter.next() {
                if e.start >= selection.begin.0 && e.start <= selection.end.0 {
                    return Some(e);
                }
            }
            return None;
        })
    }
}
