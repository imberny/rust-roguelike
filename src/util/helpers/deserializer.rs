use ron::de;
use serde::de::DeserializeOwned;

pub fn deserialize<T>(file_path: &str) -> T
where
    T: DeserializeOwned,
{
    let current_dir = std::env::current_dir().unwrap();
    let path = format!("{}/{}", current_dir.to_str().unwrap(), file_path);
    let maps = std::fs::File::open(&path).expect("Failed opening file");

    match de::from_reader(maps) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    }
}
