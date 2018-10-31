#[macro_use] extern crate serde_derive;

use std::collections::HashMap;
use toml::de::Error as TomlDeError;

#[derive(Deserialize)]
struct List {
    #[serde(rename = "radioStations")]
    radio_stations: HashMap<String, String>,
}

pub fn all() -> Result<HashMap<String, String>, TomlDeError> {
    let list = toml::from_slice::<List>(include_bytes!("../list.toml"))?;

    Ok(list.radio_stations)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use toml::Value;

    #[test]
    fn test_deser() {
        assert!(super::all().is_ok());
    }

    // `toml` has a bug[1] where deserializing to a non-Value ignores
    // duplicates.
    //
    // So, we need to deserialize to a Value here so that we can check that we
    // have none.
    //
    // Refer to the next test for verification of this.
    //
    // [1]: https://github.com/alexcrichton/toml-rs/issues/230
    #[test]
    fn test_duplicates() {
        assert!(toml::from_slice::<Value>(include_bytes!("../list.toml")).is_ok());
    }

    // Showcase that deserializing to a `Value` from a string with duplicates
    // returns an error, but deserializing to a `HashMap<String, u64>` does not
    // error.
    #[test]
    fn test_duplicates_error() {
        const INPUT: &str = "a = 1\nb = 2\na = 3";

        assert!(toml::from_str::<Value>(INPUT).is_err());
        assert!(toml::from_str::<HashMap<String, u64>>(INPUT).is_ok());
    }
}
