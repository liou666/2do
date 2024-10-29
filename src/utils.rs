use std::{fs::File, io, path::PathBuf};

static DB_NAME: &str = ".todo";

pub fn get_db_path() -> PathBuf {
    dirs::home_dir()
        .map(|mut x| {
            x.push(DB_NAME);
            x
        })
        .unwrap_or_default()
}

pub fn create_db_file() -> Result<(), io::Error> {
    let path = get_db_path();
    File::create(path)?;
    Ok(())
}

pub fn check_db_file() -> Result<(), io::Error> {
    let path = get_db_path();
    if !path.exists() {
        create_db_file()?;
    }
    Ok(())
}
