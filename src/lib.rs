#[macro_use] extern crate serde_derive;

use std::collections::HashMap;
use toml::de::Error as TomlDeError;

#[derive(Deserialize)]
struct List {
    #[serde(rename = "radioStations")]
    radio_stations: HashMap<String, String>,
}

pub fn all() -> Result<HashMap<String, String>, TomlDeError> {
    let list = toml::de::from_str::<List>(include_str!("../list.toml"))?;

    Ok(list.radio_stations)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deser() {
        assert!(super::all().is_ok());
    }
}
