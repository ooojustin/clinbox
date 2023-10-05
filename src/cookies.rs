use std::fs;
use std::error::Error;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};

const COOKIES_PATH: &str = "./cookies.json";

fn get_store() -> Result<CookieStore, Box<dyn Error>> {
    let metadata = fs::metadata(COOKIES_PATH)?;
    if !metadata.is_file() {
        return Err("Cookies path must be a file.".into())
    }

    let file = fs::File::open(COOKIES_PATH)
        .map(std::io::BufReader::new)?;

    let store = CookieStore::load_json(file)
        .unwrap_or_default();

    Ok(store)
}

pub fn get_store_mutex() -> CookieStoreMutex {
    let store = get_store()
        .unwrap_or(CookieStore::default());
    CookieStoreMutex::new(store)
}

pub fn save_store(cookies: std::sync::Arc<CookieStoreMutex>) -> Result<(), Box<dyn Error>> {
    let mut writer = std::fs::File::create(COOKIES_PATH)
        .map(std::io::BufWriter::new)?;
    let store = cookies.lock().unwrap();
    store.save_json(&mut writer).unwrap();
    Ok(())
}