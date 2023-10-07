use std::fs::{self, File};
use anyhow::{Result, anyhow};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};

const COOKIES_PATH: &str = "./cookies.json";

fn get_store() -> Result<CookieStore> {
    let metadata = fs::metadata(COOKIES_PATH)?;
    if !metadata.is_file() {
        return Err(anyhow!("Cookies path must be a file."))
    }

    let file = File::open(COOKIES_PATH)
        .map(std::io::BufReader::new)?;

    let store = CookieStore::load_json(file)
        .unwrap_or_default();

    Ok(store)
}

pub fn get_store_mutex() -> CookieStoreMutex {
    let store = get_store()
        .unwrap_or(CookieStore::default());
    //let store = match get_store() {
        //Ok(s) => s,
        //Err(e) => {
            //eprintln!("get_store error: {}", e);
            //CookieStore::default()
        //}
    //};
    CookieStoreMutex::new(store)
}

pub fn save_store(cookies: std::sync::Arc<CookieStoreMutex>) -> Result<()> {
    let mut writer = std::fs::File::create(COOKIES_PATH)
        .map(std::io::BufWriter::new)?;
    let store = cookies.lock().unwrap();
    store.save_json(&mut writer).unwrap();
    Ok(())
}

pub fn delete_file() -> Result<()> {
    let metadata = fs::metadata(COOKIES_PATH)?;
    if !metadata.is_file() {
        return Err(anyhow!("Cookies path must be a file."))
    }

    fs::remove_file(COOKIES_PATH)?;
    Ok(())
}
