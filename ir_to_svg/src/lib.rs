use usvg::Options;

use ir_to_mei::ToMei;

pub struct EngravingEngine {
    usvg_options: Options,
}

impl EngravingEngine {
    pub fn new(music_font_path: impl AsRef<std::path::Path>) -> Self {
        let mut fontdb = fontdb::Database::new();
        fontdb.load_font_file(&music_font_path).unwrap();
        let opt = Options {
            font_family: "Bravura Text".to_string(),
            fontdb,
            ..usvg::Options::default()
        };

        Self {
            usvg_options: opt
        }
    }

    pub fn to_svg(&self, line: &ir::Line) -> Tree {
        let input = format!(r#"
         <svg width="391" height="391" xmlns="http://www.w3.org/2000/svg">
             <text font-family="Bravura Text">{}</text>
         </svg>
        "#, line.to_smufl());
        Tree::from_str(&input, &self.usvg_options).unwrap()
    }
}
