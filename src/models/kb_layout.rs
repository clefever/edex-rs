use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub struct KbLayout {
    pub row_numbers: Vec<Key>,
    pub row_1: Vec<Key>,
    pub row_2: Vec<Key>,
    pub row_3: Vec<Key>,
    pub row_space: Vec<Key>,
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
