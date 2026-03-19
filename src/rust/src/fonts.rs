use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub fn load_fonts_from_dir(path: &str) -> Result<Vec<Vec<u8>>, String> {
    fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .map(|entry| {
            let entry: DirEntry = entry.map_err(|e| e.to_string())?;
            let path: PathBuf = entry.path();

            if !path.is_file() {
                return Ok(None);
            }

            let ext: &str = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let is_font: bool = matches!(ext.to_ascii_lowercase().as_str(), "ttf" | "otf" | "ttc");

            if !is_font {
                return Ok(None);
            }

            let bytes: Vec<u8> = fs::read(&path).map_err(|e| e.to_string())?;
            Ok(Some(bytes))
        })
        .filter_map(|r| match r {
            Ok(Some(bytes)) => Some(Ok(bytes)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        })
        .collect()
}
