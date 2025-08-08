// this module will be able to read and write json files
// it willl read the files in the correct format and return an output
// it will also write the files in the correct format
// this format will be stated in a documentation file

//kinda weird but we will have a json module in tihs file since i hope this is the only one that will needd it (im prolly worng)
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Read};

pub struct JsonManager;

impl JsonManager {
    pub fn write_to_json<T: Serialize>(path: &str, data: &T) -> io::Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer(file, data)?;
        Ok(())
    }

    pub fn read_from_json<T: for<'de> Deserialize<'de>>(path: &str) -> io::Result<T> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let data: T = serde_json::from_str(&content)?;
        Ok(data)
    }
}
// need module to determine if the json file is valid and in the correct format for this program to read
pub struct JsonValidator;
impl JsonValidator {
pub fn validate_json_format(path: &str) -> io::Result<bool> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        serde_json::from_str::<serde_json::Value>(&content).map(|_| true).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid JSON format"))
    }
    // a place holder for now until a specific format is defined
}