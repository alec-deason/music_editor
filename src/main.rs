use std::io::{stdout, Write};
use std::process::Command;
use std::fs::File;
use simplelog::*;

use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use operations::*;

trait InputState {
    fn handle_key(self: Box<Self>, app: &mut App, c: KeyCode) -> Box<dyn InputState>;
}

struct App {
    ctx: Context,
    should_stop: bool,
    view_dirty: bool,
}
impl Default for App {
    fn default() -> Self {
        Self {
            ctx: Context {
                score: Score::default(),
                selections: Selections(vec![Selection { begin: Location(Beat(0)), end: Location(Beat(0)) }])
            },
            should_stop: false,
            view_dirty: true,
        }
    }
}

struct Idle;
impl InputState for Idle {
    fn handle_key(self: Box<Self>, app: &mut App, c: KeyCode) -> Box<dyn InputState> {
        if c == KeyCode::Esc || c == KeyCode::Char('q') {
            app.should_stop = true;
        } else if let KeyCode::Char(c) = c {
            if let Some(p) = match c {
                'a' => Some(PitchName::A),
                'b' => Some(PitchName::B),
                'c' => Some(PitchName::C),
                'd' => Some(PitchName::D),
                'e' => Some(PitchName::E),
                'f' => Some(PitchName::F),
                'g' => Some(PitchName::G),
                _ => None,
            } {
                let operation = AppendNote {
                    note: Note {
                        pitch: Pitch {
                            class: p,
                            ..Default::default()
                        },
                        octave: Octave(4)
                    },
                    duration: Beat(4),
                    selections: None,
                };
                operation.apply(&mut app.ctx);
                app.view_dirty = true;
            }
        }
        self
    }
}

fn main() -> Result<()> {
    CombinedLogger::init(
    vec![
        WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("music_editor.log").unwrap()),
    ]
    ).unwrap();

    enable_raw_mode()?;
    let mut state:Box<InputState> = Box::new(Idle);
    let mut app = App::default();

    let mei = app.ctx.score.to_mei();
    std::fs::write("/tmp/test.svg", &mei.to_svg());
    app.view_dirty = false;

    let mut viewer = Command::new("/usr/bin/imv")
        .arg("-b")
        .arg("ffffff")
        .arg("/tmp/test.svg")
        .spawn()
        .unwrap();

    loop {
        let event = read()?;
        if let Event::Key(KeyEvent { code: k, ..}) = event {
            state = state.handle_key(&mut app, k);
        }
        log::debug!("{:?}", app.ctx);
        if app.view_dirty {
            let mei = app.ctx.score.to_mei();
            std::fs::write("/tmp/test.svg", &mei.to_svg());
            app.view_dirty = false;
        }
        if app.should_stop {
            viewer.kill();
            break
        }
    }
    disable_raw_mode()
}
