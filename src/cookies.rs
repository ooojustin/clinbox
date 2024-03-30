use anyhow::{anyhow, Result};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use std::env::temp_dir;
use std::fs;
use std::io::{BufReader, BufWriter};
use tokio::fs::File;

/// Establish path of cookies file in temp directory.
fn get_file() -> String {
    let mut file_path = temp_dir();
    file_path.push("clinbox.json");
    file_path.to_string_lossy().to_string()
}

/// Retrieve cookie store from content on disk.
async fn get_store() -> Result<CookieStore> {
    let path = get_file();
    let metadata = fs::metadata(path.clone())?;
    if !metadata.is_file() {
        return Err(anyhow!("Cookies path must be a file."));
    }

    let file = File::open(path).await?;
    let std_file = File::into_std(file).await;
    let reader = BufReader::new(std_file);

    let store = CookieStore::load_json(reader).unwrap_or_default();

    Ok(store)
}

/// Retrieve mutex to access cookie store from contents on disk.
pub async fn get_store_mutex() -> CookieStoreMutex {
    let store = get_store().await.unwrap_or(CookieStore::default());
    CookieStoreMutex::new(store)
}

/// Save provided cookie store contents to file on disk.
pub async fn save_store(cookies: std::sync::Arc<CookieStoreMutex>) -> Result<()> {
    let path = get_file();
    let file = File::create(path).await?;
    let std_file = File::into_std(file).await;
    let mut writer = BufWriter::new(std_file);

    let store = cookies.lock().unwrap();
    store.save_json(&mut writer).unwrap();
    Ok(())
}

/// Delete cookies file from path returned by cookies::get_file().
pub async fn delete_file() -> Result<()> {
    let path = get_file();
    let metadata = fs::metadata(path.clone())?;
    if !metadata.is_file() {
        return Err(anyhow!("Cookies path must be a file."));
    }

    tokio::fs::remove_file(path).await?;
    Ok(())
}
