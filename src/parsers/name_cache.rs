use serde::Deserialize;
use serde::Serialize;
use std::{collections::HashMap, error::Error, fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "nameCache:9")]
    pub name_cache: HashMap<String, NameCacheItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameCacheItem {
    #[serde(rename = "name:8")]
    pub name: String,
    #[serde(rename = "isOP:1")]
    pub is_op: i64,
    #[serde(rename = "uuid:8")]
    pub uuid: String,
}

pub fn parse<P: AsRef<Path>>(path: P) -> Result<Root, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let result = serde_json::from_str::<Root>(&text)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::Root;
    use std::fs;

    #[test]
    fn desrialize_sample() {
        let text = fs::read_to_string("./sample/1/NameCache.json").unwrap();
        let result = serde_json::from_str::<Root>(&text).unwrap();
        for q in result.name_cache {
            println!("{:?}", q);
        }
    }
}
