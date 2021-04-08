use std::process::{Command, Stdio};
use strong_xml::{XmlRead, XmlWrite};
use tempfile::tempdir;

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "mei")]
pub struct Mei {
    #[xml(attr = "meiversion")]
    pub meiversion: Option<String>,
    #[xml(attr = "resp")]
    pub resp: Option<String>,
    #[xml(child = "meiHead")]
    pub mei_head: Option<MeiHead>,
    #[xml(child = "music")]
    pub music: Option<Music>,
}

impl Mei {
    pub fn to_svg(&self) -> String {
        let dir = tempdir().unwrap();

        let file_path = dir.path().join("score.mei");
        std::fs::write(&file_path, self.to_string().unwrap()).unwrap();
        Command::new("/usr/local/bin/verovio")
            .arg("--footer")
            .arg("none")
            .arg(&file_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();
        let file_path = dir.path().join("score.svg");

        let result = std::fs::read_to_string(&file_path).unwrap();
        dir.close().unwrap();
        result
    }
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "meiHead")]
pub struct MeiHead {
    //TODO: complete
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "music")]
pub struct Music {
    //TODO: complete
    #[xml(child = "body")]
    pub body: Option<Body>
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "body")]
pub struct Body {
    //TODO: complete
    #[xml(child = "mdiv")]
    pub mdivs: Vec<MDiv>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "mdiv")]
pub struct MDiv {
    //TODO: complete
    #[xml(child = "score")]
    pub score: Option<Score>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "score")]
pub struct Score {
    //TODO: complete
    #[xml(child = "scoreDef")]
    pub score_def: Option<ScoreDef>,
    #[xml(child = "section")]
    pub sections: Vec<Section>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "scoreDef")]
pub struct ScoreDef {
    //TODO: complete
    #[xml(attr = "meter.count")]
    pub meter_count: Option<u32>,
    #[xml(attr = "meter.unit")]
    pub meter_unit: Option<u32>,
    #[xml(attr = "key.sig")]
    pub key_sig: Option<String>,
    #[xml(attr = "key.mode")]
    pub key_mode: Option<String>,
    #[xml(child = "staffGrp")]
    pub staff_grp: Option<StaffGrp>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "staffGrp")]
pub struct StaffGrp {
    //TODO: complete
    #[xml(child = "staffDef")]
    pub staff_def: Option<StaffDef>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "staffDef")]
pub struct StaffDef {
    #[xml(attr = "n")]
    pub n: Option<u32>,
    #[xml(attr = "clef.line")]
    pub clef_line: Option<u32>,
    #[xml(attr = "clef.shape")]
    pub clef_shape: Option<String>,
    #[xml(attr = "clef.dis")]
    pub clef_dis: Option<u32>,
    #[xml(attr = "clef.dis.place")]
    pub clef_dis_place: Option<String>,
    #[xml(attr = "lines")]
    pub lines: Option<u32>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "section")]
pub struct Section {
    //TODO: complete
    #[xml(child = "measure")]
    pub measures: Vec<Measure>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "measure")]
pub struct Measure {
    //TODO: complete
    #[xml(attr = "n")]
    pub n: Option<u32>,
    #[xml(child = "staff")]
    pub staves: Vec<Staff>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "beam")]
pub struct Beam {
    #[xml(child = "note", child = "note", child = "rest", child = "chord", child = "beam")]
    pub events: Vec<EventLike>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "staff")]
pub struct Staff {
    //TODO: complete
    #[xml(child = "layer")]
    pub layers: Vec<Layer>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "layer")]
pub struct Layer {
    //TODO: complete
    #[xml(attr = "n")]
    pub n: Option<u32>,
    #[xml(child = "note", child = "note", child = "rest", child = "chord", child = "beam")]
    pub events: Vec<EventLike>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "chord")]
pub struct Chord {
    #[xml(attr = "dur")]
    pub dur: Option<u32>,
    #[xml(child = "note")]
    pub notes: Vec<Note>,
}

#[derive(Debug, XmlWrite, XmlRead, PartialEq, Eq)]
pub enum EventLike {
    #[xml(tag = "note")]
    Note(Note),
    #[xml(tag = "rest")]
    Rest(Rest),
    #[xml(tag = "chord")]
    Chord(Chord),
    #[xml(tag = "beam")]
    Beam(Beam),
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "note")]
pub struct Note {
    //TODO: complete
    #[xml(attr = "xml:id")]
    pub xml_id: Option<String>,
    #[xml(attr = "pname")]
    pub pname: Option<String>,
    #[xml(attr = "pclass")]
    pub pclass: Option<u32>,
    #[xml(attr = "oct")]
    pub oct: u32,
    #[xml(attr = "dur")]
    pub dur: Option<u32>,
}

#[derive(Debug, Default, XmlWrite, XmlRead, PartialEq, Eq)]
#[xml(tag = "rest")]
pub struct Rest {
    //TODO: complete
    #[xml(attr = "dur")]
    pub dur: Option<u32>,
}
