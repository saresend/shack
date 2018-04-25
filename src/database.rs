use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use toml;
#[derive(Serialize, Deserialize, Debug)]
struct BlToml {
    elements: Vec<toml::value::Value>,
}

pub fn delete_value(key: &str) {
    let mut file = get_data_file();

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            if let Ok(mut data) = toml::from_str::<BlToml>(&contents) {
                data.elements.retain(|x| x[key].as_str().is_none());
                println!("Parsed form: {:?}", toml::to_string(&data).unwrap());
                // file.write_all(toml::to_string(&data).unwrap().as_bytes()).expect("Unexpectedly failed to write to data file. Please file a bug report to http://github.com/saresend/shack");
                return;
            }
            println!("Couldn't delete value");
        }
        Err(_) => println!("Couldn't delete value"),
    };
}

#[test]
fn delete_test() {
    save_value("digital_ocean_ip", "123.123.123.123");
    assert_eq!(
        get_value("digital_ocean_ip"),
        Some("123.123.123.123".to_string()),
    );
    delete_value("digital_ocean_ip");
    assert_eq!(get_value("digital_ocean_ip"), None);
}

pub fn save_value(key: &str, value: &str) {
    let mut file = get_data_file();

    file.write_all(&format!("\n[[elements]]\n{}=\"{}\"", key, value).as_bytes())
        .unwrap();
}

#[test]
fn test_save() {
    save_value("hello", "okay");
    assert_eq!(get_value("hello"), Some("okay".to_string()));
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
            println!("{:?}", element);
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
    if let Ok(file) = OpenOptions::new().read(true).append(true).open("data.toml") {
        file
    } else {
        File::create("data.toml").unwrap();
        OpenOptions::new()
            .read(true)
            .append(true)
            .open("data.toml")
            .unwrap()
    }
}
