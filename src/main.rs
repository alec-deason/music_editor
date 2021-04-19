use std::io::{stdout, Write};
use std::process::Command;
use std::fs::File;
use simplelog::*;
use strong_xml::XmlWrite;

use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use operations::*;

trait InputState {
    fn handle_key(self: Box<Self>, app: &mut App, c: KeyCode, m: KeyModifiers) -> Box<dyn InputState>;
}

struct App {
    ctx: Context,
    should_stop: bool,
    view_dirty: bool,
}
impl Default for App {
    fn default() -> Self {
        Self {
            ctx: Context::default(),
            should_stop: false,
            view_dirty: true,
        }
    }
}

struct Idle;
impl InputState for Idle {
    fn handle_key(self: Box<Self>, app: &mut App, c: KeyCode, m: KeyModifiers) -> Box<dyn InputState> {
        if c == KeyCode::Esc || c == KeyCode::Char('q') {
            app.should_stop = true;
        } else if c == KeyCode::Left {
            if m.contains(KeyModifiers::SHIFT) {
                MoveSelectionsEnd {
                    delta: Duration::Event(-1),
                    selections: vec![0],
                }.apply(&mut app.ctx);
            } else {
                MoveSelections {
                    delta: Duration::Event(-1),
                    selections: vec![0],
                }.apply(&mut app.ctx);
            }
            app.view_dirty = true;
        } else if c == KeyCode::Right {
            if m.contains(KeyModifiers::SHIFT) {
                MoveSelectionsEnd {
                    delta: Duration::Event(1),
                    selections: vec![0],
                }.apply(&mut app.ctx);
            } else {
                MoveSelections {
                    delta: Duration::Event(1),
                    selections: vec![0],
                }.apply(&mut app.ctx);
            }
            app.view_dirty = true;
        } else if c == KeyCode::Backspace {
            DeleteSelections {
                selections: vec![0],
            }.apply(&mut app.ctx);
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
                    duration: Pulse(4),
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

    let mut verovio = verovio::Verovio::new("/usr/local/share/verovio/");

    enable_raw_mode()?;
    let mut state:Box<InputState> = Box::new(Idle);
    let mut app = App::default();

    let mei = app.ctx.score.to_mei();
    let mei_xml = mei.to_string().unwrap();
    let svg = verovio.render_data(&mei_xml);
    let package =  sxd_document::parser::parse(&svg).unwrap();
    let mut context = sxd_xpath::Context::new();
    context.set_namespace("svg", "http://www.w3.org/2000/svg");
    let factory = sxd_xpath::Factory::new();
    let mut doc = package.as_document();
    let mut f = std::fs::File::create("/tmp/test.svg").unwrap();
    sxd_document::writer::format_document(&doc, &mut f).unwrap();
    app.view_dirty = false;

    let mut viewer = Command::new("/usr/bin/imv")
        .arg("-b")
        .arg("ffffff")
        .arg("/tmp/test.svg")
        .spawn()
        .unwrap();

    loop {
        let event = read()?;
        if let Event::Key(KeyEvent { code: k, modifiers: m}) = event {
            state = state.handle_key(&mut app, k, m);
        }
        log::debug!("{:?}", app.ctx);
        if app.view_dirty {
            let mei = app.ctx.score.to_mei();
            let mei_xml = mei.to_string().unwrap();
            let svg = verovio.render_data(&mei_xml);
            let package =  sxd_document::parser::parse(&svg).unwrap();
            let mut doc = package.as_document();
            for e in app.ctx.events_in_selection(0) {
                let xpath = factory.build(&format!("//svg:g[@id='note_{}']/svg:g/svg:use", e.id())).unwrap().unwrap();
                let value = xpath.evaluate(&context, doc.root()).unwrap();

                if let sxd_xpath::Value::Nodeset(ns) = value {
                    for node in ns {
                        if let sxd_xpath::nodeset::Node::Element(e) = node {
                            e.set_attribute_value("fill", "red");
                        }
                    }
                }


            }
            let mut f = std::fs::File::create("/tmp/test.svg").unwrap();
            sxd_document::writer::format_document(&doc, &mut f).unwrap();
            Command::new("/usr/bin/imv-msg")
                .arg(&format!("{}", viewer.id()))
                .arg("open")
                .arg("/tmp/test.svg")
                .status();
            app.view_dirty = false;
        }
        if app.should_stop {
            viewer.kill();
            break
        }
    }
    disable_raw_mode()
}
