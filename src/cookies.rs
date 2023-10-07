use std::fs::{self, File};
use std::env::temp_dir;
use anyhow::{Result, anyhow};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};

/// Establish path of cookies file in temp directory.
fn get_file() -> String {
    let mut file_path = temp_dir();
    file_path.push("clinbox.json");
    file_path.to_string_lossy().to_string()
}

/// Retrieve cookie store from content on disk.
fn get_store() -> Result<CookieStore> {
    let path = get_file();
    let metadata = fs::metadata(path.clone())?;
    if !metadata.is_file() {
        return Err(anyhow!("Cookies path must be a file."))
    }

    let file = File::open(path)
        .map(std::io::BufReader::new)?;

    let store = CookieStore::load_json(file)
        .unwrap_or_default();

    Ok(store)
}

/// Retrieve mutex to access cookie store from contents on disk.
pub fn get_store_mutex() -> CookieStoreMutex {
    let store = get_store()
        .unwrap_or(CookieStore::default());
    CookieStoreMutex::new(store)
}

/// Save provided cookie store contents to file on disk.
pub fn save_store(cookies: std::sync::Arc<CookieStoreMutex>) -> Result<()> {
    let path = get_file();
    let mut writer = std::fs::File::create(path)
        .map(std::io::BufWriter::new)?;
    let store = cookies.lock().unwrap();
    store.save_json(&mut writer).unwrap();
    Ok(())
}

/// Delete cookies file from path returned by cookies::get_file().
pub fn delete_file() -> Result<()> {
    let path = get_file();
    let metadata = fs::metadata(path.clone())?;
    if !metadata.is_file() {
        return Err(anyhow!("Cookies path must be a file."))
    }

    fs::remove_file(path)?;
    Ok(())
}
