use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub struct KbLayout {
    pub row_numbers: Vec<Key>,
    pub row_1: Vec<Key>,
    pub row_2: Vec<Key>,
    pub row_3: Vec<Key>,
    pub row_space: Vec<Key>,
}

impl KbLayout {
    /// Parse via `Value` so duplicate keys resolve last-wins like JS
    /// `JSON.parse` (e.g. da-DK.json has a duplicate `alt_cmd`).
    pub fn from_json_str(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_value(serde_json::from_str(raw)?)
    }
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Key {
    pub name: String,
    pub cmd: String,
    pub shift_name: Option<String>,
    pub shift_cmd: Option<String>,
    pub ctrl_cmd: Option<String>,
    pub alt_name: Option<String>,
    pub alt_cmd: Option<String>,
    pub altshift_name: Option<String>,
    pub altshift_cmd: Option<String>,
    pub fn_name: Option<String>,
    pub fn_cmd: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_bundled_layouts_deserialize() {
        let layouts_dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/kb_layouts");
        let mut count = 0;
        for entry in std::fs::read_dir(&layouts_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).unwrap();
            let layout = KbLayout::from_json_str(&raw)
                .unwrap_or_else(|e| panic!("{} failed to deserialize: {e}", path.display()));
            for (row, keys) in [
                ("row_numbers", &layout.row_numbers),
                ("row_1", &layout.row_1),
                ("row_2", &layout.row_2),
                ("row_3", &layout.row_3),
                ("row_space", &layout.row_space),
            ] {
                assert!(!keys.is_empty(), "{}: {row} is empty", path.display());
                for key in keys {
                    // name may be empty upstream (blank space/enter keys),
                    // but a key without a command can never emit anything
                    assert!(
                        !key.cmd.is_empty(),
                        "{}: {row} has a key with an empty cmd",
                        path.display()
                    );
                }
            }
            count += 1;
        }
        assert!(
            count > 0,
            "no layout JSONs found in {}",
            layouts_dir.display()
        );
    }
}
