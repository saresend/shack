use serde_json;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
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
    let mut file = get_data_file(false);

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            if let Ok(mut data) = serde_json::from_str::<Data>(&contents) {
                data.elements.retain(|x| x.key != key);
                file.write_all(serde_json::to_string(&data).unwrap().as_bytes())
                    .unwrap();
            }
            println!("Couldn't delete value");
        }
        Err(e) => println!("Couldn't delete value {}", e),
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
    let mut file = get_data_file(false);
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if let Ok(mut database) = serde_json::from_str::<Data>(&contents) {
        database.elements.push(KeyVal {
            key: String::from(key),
            val: String::from(value),
        });
        file.set_len(0).unwrap();
        file.write_all(serde_json::to_string(&database).unwrap().as_bytes())
            .unwrap();
    }
    println!("Couldn't parse");
}

#[test]
fn test_save() {
    save_value("hello", "okay");
    assert_eq!(get_value("hello"), Some("okay".to_string()));
}

pub fn get_value(key: &str) -> Option<String> {
    let mut file = get_data_file(true);

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => return None,
    };
    if let Ok(database) = serde_json::from_str::<Data>(&contents) {
        for element in database.elements {
            if element.key == key {
                return Some(element.val);
            }
        }
    }
    None
}

pub fn print_all_values() {
    let mut file = get_data_file(true);
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

#[test]
pub fn test_print_all_values() {
    print_all_values();
}
fn get_data_file(should_append: bool) -> File {
    if let Ok(file) = OpenOptions::new()
        .read(true)
        .write(true)
        .append(should_append)
        .open("data.json")
    {
        file
    } else {
        File::create("data.json").unwrap();
        OpenOptions::new()
            .read(true)
            .write(true)
            .append(should_append)
            .open("data.json")
            .unwrap()
    }
}
