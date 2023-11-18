use super::return_error;
use chrono::{Datelike, Local, Timelike};
use json::{self, JsonValue};
use std::{
    fs::{self, File},
    io::Write,
};

const DATA_FILE_PATH: &str = "data.json";
pub fn parse_data() -> JsonValue {
    return json::parse(fs::read_to_string(DATA_FILE_PATH).unwrap().as_str()).unwrap_or_else(
        |_err| {
            return_error!(
                "Could not parse data you might need to clear your data usind clear command"
            )
        },
    );
}

pub fn save_data(data: JsonValue) -> Result<(), &'static str> {
    fs::write(DATA_FILE_PATH, json::stringify(data)).expect("Could not save the file");
    return Ok(());
}

pub fn reset_data() -> Result<(), &'static str> {
    if file_exists(DATA_FILE_PATH) {
        fs::remove_file(DATA_FILE_PATH).ok();
    };

    match File::create(DATA_FILE_PATH) {
        Ok(mut file) => {
            if let Err(_err) = file.write_all("{\"id_cursor\": 0, \"todos\": []}".as_bytes()) {
                return Err("Error writing to file");
            }
        }
        Err(_err) => return Err("Error creating file"),
    }

    Ok(())
}

pub fn file_exists(file_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        metadata.is_file()
    } else {
        false
    }
}

pub fn current_time() -> String {
    let local_time = Local::now();

    let year = local_time.year();
    let month = local_time.month();
    let day = local_time.day();
    let hour = local_time.hour();
    let minute = local_time.minute();

    let time = format!("{}-{:02}-{:02} {:02}:{:02}", year, month, day, hour, minute);
    return time;
}
