use crate::basic::Exception;
use reqwest::blocking::Client as SyncClient;

pub fn sync_get_bytes(url: &str) -> Result<Vec<u8>, Exception> {
    let client = SyncClient::new();
    let response = client.get(url).send().map_err(|e| Exception::new(0, e.to_string()))?;

    // 检查 HTTP 响应是否成功
    if !response.status().is_success() {
        return Err(Exception::new(100, format!("HTTP Failed: {}", response.status())));
    }

    // 尝试获取响应的字节
    let bytes = response.bytes().map_err(|e| Exception::new(400, e.to_string()))?;
    Ok(bytes.to_vec())
}

pub fn sync_get_text(url: &str) -> Result<String, Exception> {
    let client = SyncClient::new();
    let response = client.get(url).send().map_err(|e| Exception::new(0, e.to_string()))?;

    // 检查 HTTP 响应是否成功
    if !response.status().is_success() {
        return Err(Exception::new(101, format!("HTTP Failed: {}", response.status())));
    }

    // 尝试获取响应的文本内容
    let body = response.text().map_err(|e| Exception::new(400, e.to_string()))?;
    Ok(body)
}
