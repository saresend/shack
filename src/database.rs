use std::fs::File;
use std::io::Read;
use toml;

#[derive(Serialize, Deserialize)]
struct BlToml {
    elements: Vec<toml::value::Value>,
}

pub fn save_value(key: &str, value: &str) {
    let file = get_data_file();
}
pub fn get_value(key: &str) -> Option<String> {
    let mut file = get_data_file();

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => return None,
    };
    println!("{}", contents);
    if let Ok(database) = toml::from_str::<BlToml>(&contents) {
        for element in database.elements {
            match element[key].as_str() {
                Some(val) => return Some(val.to_string()),
                None => {}
            };
        }
        None
    } else {
        None
    }
}

#[test]
pub fn test_get_value() {
    get_value("key1");
}
pub fn get_all_values() -> Vec<String> {
    unimplemented!();
}
fn get_data_file() -> File {
    if let Ok(file) = File::open("data.toml") {
        file
    } else {
        File::create("data.toml").unwrap();
        File::open("data.toml").unwrap()
    }
}
