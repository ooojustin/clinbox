use std::process::Command;
use reqwest::header;

// Most common user agents: https://www.useragents.me/
// Chrome 117.0.0 [Windows]
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36";

pub fn headers(xml_http: bool) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static(USER_AGENT));
    if xml_http {
        headers.insert("X-Requested-With", header::HeaderValue::from_static("XMLHttpRequest"));
    }
    headers
}

pub fn open(path: String) {
    let is_windows = cfg!(target_os = "windows");

    let command = if is_windows {
        format!("rundll32 url.dll,FileProtocolHandler {}", path)
    } else if cfg!(target_os = "macos") {
        format!("open '{}'", path)
    } else {
        format!("xdg-open '{}'", path)
    };

    let shell_command = if is_windows { "cmd" } else { "sh" };
    let shell_arg = if is_windows { "/c" } else { "-c" };

    let result = Command::new(shell_command)
        .arg(shell_arg)
        .arg(&command)
        .status();

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }
}
