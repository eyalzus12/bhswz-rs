use regex::Regex;
use std::sync::LazyLock;

const LEVEL_DESC_PATTERN: &str = r#"^<LevelDesc AssetDir=".+?"\s+LevelName="(.+?)".*?>"#;
const CUTSCENE_TYPE_PATTERN: &str = r#"^<CutsceneType CutsceneName="(.+?)".*?>"#;
const GENERAL_XML_PATTERN: &str = r#"<(\w+)>"#;
const GENERAL_CSV_PATTERN: &str = r#"^(\w+)\r?\n"#;

fn make_regex(re: &str) -> Regex {
    Regex::new(re).unwrap()
}
const LEVEL_DESC_REGEX: LazyLock<Regex> = LazyLock::new(|| make_regex(LEVEL_DESC_PATTERN));
const CUTSCENE_TYPE_REGEX: LazyLock<Regex> = LazyLock::new(|| make_regex(CUTSCENE_TYPE_PATTERN));
const GENERAL_XML_REGEX: LazyLock<Regex> = LazyLock::new(|| make_regex(GENERAL_XML_PATTERN));
const GENERAL_CSV_REGEX: LazyLock<Regex> = LazyLock::new(|| make_regex(GENERAL_CSV_PATTERN));

pub fn get_swz_file_name(file_content: &str) -> Option<String> {
    if let Some(captures) = LEVEL_DESC_REGEX.captures(file_content) {
        let level_name = &captures[1];
        return Some(format!("LevelDesc_{level_name}.xml"));
    }

    if let Some(captures) = CUTSCENE_TYPE_REGEX.captures(file_content) {
        let cutscene_name = &captures[1];
        return Some(format!("CutsceneType_{cutscene_name}.xml"));
    }

    if let Some(captures) = GENERAL_XML_REGEX.captures(file_content) {
        let xml_name = &captures[1];
        return Some(format!("{xml_name}.xml"));
    }

    if let Some(captures) = GENERAL_CSV_REGEX.captures(file_content) {
        let csv_name = &captures[1];
        return Some(format!("{csv_name}.csv"));
    }

    return None;
}
