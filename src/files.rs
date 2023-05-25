use std::time::SystemTime;
use std::{fs, error::Error};
use std::env;

pub fn save_result(contents: Vec<&str>) -> Result<&str, Box<dyn Error>> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!");
    let save_dir = env::temp_dir();
    let file_name = format!("{}.txt", now.as_secs().to_string());

    let full_path = format!("{}{}", save_dir.to_str().unwrap(), file_name);
    fs::File::create(&full_path)?;
    for content in contents.iter() {
        fs::write(&full_path, format!("{}\r\n", content))?;
    }

    Ok("Arquivo criado")
}
