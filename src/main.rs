#[macro_use]
 extern crate magic_crypt;
 extern  crate passwords;
 use std::{collections::HashMap, fs};




 fn load_data() -> HashMap<String, String> {
	let data::String = fs::read_to_string("db.json").unwrap_or_else(|_|  "{}".to_string());
    serde_json::from_str(&data).unwrap_or_default()
 }