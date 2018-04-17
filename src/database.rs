use std::fs::File;
use std::io::{Read, Write};
use toml;

#[derive(Serialize, Deserialize)]
struct BlToml {
    elements: Vec<toml::value::Value>,
}

pub fn save_value(key: &str, value: &str) {
    let mut file = get_data_file();
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => {
            println!("Couldn't save value");
        }
    };

    contents += &format!("\n[[elements]]\n{}=\"{}\"", key, value);

    file.write_all(&contents.as_bytes());
}
pub fn get_value(key: &str) -> Option<String> {
    let mut file = get_data_file();

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => return None,
    };
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
    assert_eq!(Some("blessed".to_string()), get_value("key1"));
}
pub fn print_all_values() {
    let mut file = get_data_file();
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => println!("Couldn't find database file"),
    };

    if let Ok(db) = toml::from_str::<BlToml>(&contents) {
        for element in db.elements {
            print!("{}", element);
        }
    }
}

#[test]
pub fn test_print_all_values() {
    print_all_values();
}
fn get_data_file() -> File {
    if let Ok(file) = File::open("data.toml") {
        file
    } else {
        File::create("data.toml").unwrap();
        File::open("data.toml").unwrap()
    }
}
