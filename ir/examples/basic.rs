use std::process::Command;
use usvg::SystemFontDB;
use strong_xml::XmlWrite;
use ir::*;

fn main() {
    let mut mei = Mei::default();
    mei.mei_head = Some(MeiHead {
        ..Default::default()
    });
    let mut layer = Layer::default();
    layer.n = Some(1);
    layer.events.extend(vec![
        EventLike::Note(Note {
            xml_id: Some("note-1".to_string()),
            pname: "c".to_string(),
            oct: 4,
            dur: Some(4)
        }),
        EventLike::Note(Note {
            xml_id: Some("note-2".to_string()),
            pname: "d".to_string(),
            oct: 4,
            dur: Some(4)
        }),
        EventLike::Note(Note {
            xml_id: Some("note-3".to_string()),
            pname: "e".to_string(),
            oct: 4,
            dur: Some(4)
        }),
        EventLike::Beam(Beam {
            events: vec![
                EventLike::Note(Note {
                xml_id: Some("note-4".to_string()),
                pname: "f".to_string(),
                oct: 4,
                dur: Some(8)
                }),
                EventLike::Note(Note {
                    xml_id: Some("note-5".to_string()),
                    pname: "g".to_string(),
                    oct: 4,
                    dur: Some(8)
                }),
            ],
        })
    ]);
    mei.music = Some(Music {
        body: Some(Body {
            mdivs: vec![
                MDiv {
                    score: Some(Score {
                        score_def: Some(ScoreDef {
                            meter_count: Some(4),
                            meter_unit: Some(4),
                            key_sig: Some("0".to_string()),
                            key_mode: Some("major".to_string()),
                            staff_grp: Some(StaffGrp {
                                staff_def: Some(StaffDef {
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
                        sections: vec![Section {
                            measures: vec![Measure {
                                n: Some(1),
                                staves: vec![Staff {
                                    layers: vec![layer],
                                    ..Default::default()
                                }],
                                ..Default::default()
                            }],
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
        }),
        ..Default::default()
    });

    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();
    opt.fontdb.set_generic_families();

    let rtree = usvg::Tree::from_str(&mei.to_svg(), &opt).unwrap();
    let mut pixmap = tiny_skia::Pixmap::new(1920, 1080).unwrap();
    resvg::render(&rtree, usvg::FitTo::Width(1080), pixmap.as_mut()).unwrap();
    pixmap.save_png("/tmp/test.png").unwrap();
}
