use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use typst::text::Font;
use typst_kit::fonts::{FontSearcher, Fonts};

static DEFAULT_FONTS: OnceLock<Vec<Font>> = OnceLock::new();

pub fn load_fonts(font_path: Option<&str>, ignore_system_fonts: bool) -> Result<Vec<Font>, String> {
    let custom_dir: Option<PathBuf> = font_path.map(validate_font_dir).transpose()?;

    let mut fonts: Vec<Font> = custom_dir
        .map(|dir| search_fonts(vec![dir], ignore_system_fonts))
        .unwrap_or_default();

    fonts.extend(DEFAULT_FONTS.get_or_init(default_fonts).clone());

    Ok(fonts)
}

fn default_fonts() -> Vec<Font> {
    search_fonts(vec![], true)
}

fn search_fonts(font_dirs: Vec<PathBuf>, ignore_system_fonts: bool) -> Vec<Font> {
    let mut searcher = FontSearcher::new();
    searcher
        .include_system_fonts(!ignore_system_fonts)
        .include_embedded_fonts(true);

    let Fonts { fonts, .. } = searcher.search_with(font_dirs);
    fonts.into_iter().filter_map(|slot| slot.get()).collect()
}

fn validate_font_dir(path: &str) -> Result<PathBuf, String> {
    if path.trim().is_empty() {
        return Err("`font_path` must not be an empty path".to_owned());
    }

    let path: &Path = Path::new(path);
    if !path.is_dir() {
        return Err(format!("Font directory does not exist: {}", path.display()));
    }

    Ok(path.to_path_buf())
}
