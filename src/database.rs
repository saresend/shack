use serde_json;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::io::{Seek, SeekFrom};

#[derive(Serialize, Deserialize, Debug)]
struct KeyVal {
    pub key: String,
    pub val: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    pub elements: Vec<KeyVal>,
}

pub fn delete_value(key: &str) {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .open("data.json")
        .unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents).unwrap();
    println!("Trying to parse: {}", contents);
    let mut data: Data = serde_json::from_str(&contents).unwrap();
    data.elements.retain(|x| x.key != key);
    f.set_len(0).unwrap(); //Hopefully clear the file
    f.seek(SeekFrom::Start(0)).unwrap();
    f.write_all(serde_json::to_string(&data).unwrap().as_bytes())
        .unwrap();
    f.write(b"\n").unwrap();
}

#[test]
fn delete_test() {
    delete_value("digital_ocean_ip");
    assert_eq!(get_value("digital_ocean_ip"), None);
}

pub fn save_value(key: &str, value: &str) {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .open("data.json")
        .unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents).unwrap();
    let mut data_obj: Data = serde_json::from_str(&contents).unwrap();
    data_obj.elements.push(KeyVal {
        key: String::from(key),
        val: String::from(value),
    });

    f.set_len(0).unwrap(); //Hopefully clear the file
    f.seek(SeekFrom::Start(0)).unwrap();
    f.write_all(serde_json::to_string(&data_obj).unwrap().as_bytes())
        .unwrap();
    f.write(b"\n").unwrap();
}

#[test]
fn test_save() {
    initialize();
    save_value("hello", "okay");
    assert_eq!(get_value("hello"), Some("okay".to_string()));
}

pub fn get_value(key: &str) -> Option<String> {
    let mut f = OpenOptions::new().read(true).open("data.json").unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents).unwrap();
    println!("Trying to parse: {}", contents);
    let data: Data = serde_json::from_str(&contents).unwrap();
    for element in data.elements.iter() {
        if element.key == key {
            return Some(element.val.to_string());
        }
    }
    None
}

pub fn print_all_values() {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("data.json")
        .unwrap();
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => println!("Couldn't find database file"),
    };

    if let Ok(db) = serde_json::from_str::<Data>(&contents) {
        for element in db.elements {
            print!("{} : {}", element.key, element.val);
        }
    }
}
pub fn initialize() {
    if let Err(_) = fs::metadata("data.json") {
        let cont = Data { elements: vec![] };
        let mut f = File::create("data.json").unwrap();

        f.write_all(serde_json::to_string(&cont).unwrap().as_bytes())
            .unwrap();
    }
}

#[test]
pub fn test_print_all_values() {
    print_all_values();
}
