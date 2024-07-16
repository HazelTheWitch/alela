use chrono::NaiveDate;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub date: NaiveDate,
    pub version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File<T> {
    pub meta: Meta,
    pub data: T,
}

#[cfg(test)]
pub mod test_utils {
    use std::{fs, io::BufReader, path::PathBuf};

    use lazy_static::lazy_static;
    use serde::de::DeserializeOwned;

    use crate::File;

    const BASE_PATH: &str = "https://mtgjson.com/api/v5/";

    fn cache_dir() -> PathBuf {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".cache");

        fs::create_dir(&dir).unwrap();
        
        dir
    }
    
    lazy_static! {
        pub static ref CACHE_DIR: PathBuf = cache_dir();
    }

    pub async fn deserialize_file<T: DeserializeOwned>(file: &str) -> File<T> {
        let file_path = CACHE_DIR.join(&file);

        if !file_path.exists() {
            let data = reqwest::get(&format!("{BASE_PATH}{file}"))
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap();

            fs::write(&file_path, &data).unwrap();   
        }

        let file = BufReader::new(fs::File::open(&file_path).unwrap());

        return serde_json::from_reader(file).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_utils::deserialize_file, Meta};

    #[tokio::test]
    async fn meta() {
        deserialize_file::<Meta>("Meta.json").await;
    }
}
