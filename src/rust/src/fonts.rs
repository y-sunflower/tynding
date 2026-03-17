use std::fs;

pub fn load_fonts_from_dir(path: &str) -> Result<Vec<Vec<u8>>, String> {
    fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .map(|entry| {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if !path.is_file() {
                return Ok(None);
            }

            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let is_font = matches!(ext.to_ascii_lowercase().as_str(), "ttf" | "otf" | "ttc");

            if !is_font {
                return Ok(None);
            }

            let bytes = fs::read(&path).map_err(|e| e.to_string())?;
            Ok(Some(bytes))
        })
        .filter_map(|r| match r {
            Ok(Some(bytes)) => Some(Ok(bytes)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        })
        .collect()
}
