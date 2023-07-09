use std::collections::HashMap;

use fancy_regex::{Regex, Error};

const FORMAT_CHECK_REGEX: &str = r"^(\d+s)?\s?(\d+m)?\s?(\d+h)?\s?(\d+D)?\s?(\d+M)?\s?(\d+Y)?$";
const EXTRACT_CHARS_REGEX: &str = r"(?<=\d)[smhDMY]";
const EXTRACT_NUMS_REGEX: &str = r"\d+(?=[a-zA-z])";

pub fn time_string_to_seconds(timestamp_str: String) -> Result<i64, Error> {
    let check_format = Regex::new(FORMAT_CHECK_REGEX)?;

    if !check_format.is_match(&timestamp_str)? {
        return Err(Error::ParseError(0, fancy_regex::ParseError::GeneralParseError(String::from("Incorrect format"))));
    }

    let get_chars = Regex::new(EXTRACT_CHARS_REGEX)?;

    let char_list: Vec<&str> = get_chars.find_iter(&timestamp_str).map(|i| { 
        match i {
            Ok(m) => m.as_str(),
            Err(e) => {
                log::error!("REGEX ERROR: {:#?}", e);
                "0"
            }
        }
    }).collect();

    let get_nums = Regex::new(EXTRACT_NUMS_REGEX)?;

    let num_list: Vec<&str> = get_nums.find_iter(&timestamp_str).map(|i| { 
        match i {
            Ok(m) => m.as_str(),
            Err(e) => {
                log::error!("REGEX ERROR: {:#?}", e);
                "0"
            }
        }
    }).collect();

    let hash_numbers: HashMap<&str, &str> = char_list.into_iter().zip(num_list.into_iter()).collect();

    let char_multiplier: HashMap<&str, u32> = HashMap::from([
        ("s", 1),
        ("m", 60),
        ("h", 3600),
        ("D", 86400),
        ("M", 2592000), // 30 days
        ("Y", 31557600)
    ]);

    let mut timestamp_seconds: i64 = 0;

    for (time_type, value) in hash_numbers {
        if let Some(multiplier) = char_multiplier.get(time_type) {
            if let Ok(v) = value.parse::<u32>() {
                timestamp_seconds += (v * multiplier) as i64;
            };
        };
    }

    Ok(timestamp_seconds)
}