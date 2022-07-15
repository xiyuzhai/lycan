use std::path::PathBuf;

pub struct LycanWordInterner {
    words: Vec<()>,
}

impl LycanWordInterner {
    pub fn new() -> LycanWordInterner {
        let lycan_dir: PathBuf = std::env::var("LYCAN_DIR").unwrap().into();
        let words_json_path = lycan_dir.join("data/words.json");
        let words_json_text = std::fs::read_to_string(words_json_path).unwrap();
        let words = serde_json::from_str(&words_json_text).unwrap();
        Self { words }
    }
}
