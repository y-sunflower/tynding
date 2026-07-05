use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use typst::text::Font;
use typst_kit::fonts::{embedded, scan, system, FontSource};

static DEFAULT_FONTS: OnceLock<Vec<Font>> = OnceLock::new();
static DEFAULT_FONTS_WITHOUT_SYSTEM: OnceLock<Vec<Font>> = OnceLock::new();

pub fn load_fonts(font_path: Option<&str>, ignore_system_fonts: bool) -> Result<Vec<Font>, String> {
    let custom_dir: Option<PathBuf> = font_path.map(validate_font_dir).transpose()?;

    // Custom fonts are prepended so Typst's font matching sees package/user
    // supplied fonts before the broader system and embedded font set.
    let mut fonts: Vec<Font> = custom_dir
        .map(|dir| search_fonts(vec![dir], false, false))
        .unwrap_or_default();

    fonts.extend(default_fonts(ignore_system_fonts).clone());

    Ok(fonts)
}

fn default_fonts(ignore_system_fonts: bool) -> &'static Vec<Font> {
    // Scanning system fonts can be expensive, especially when R users compile
    // many documents in one session. Keep separate caches because
    // `ignore_system_fonts = TRUE` must be reproducible across machines.
    if ignore_system_fonts {
        DEFAULT_FONTS_WITHOUT_SYSTEM.get_or_init(|| search_fonts(vec![], false, true))
    } else {
        DEFAULT_FONTS.get_or_init(|| search_fonts(vec![], true, true))
    }
}

fn search_fonts(
    font_dirs: Vec<PathBuf>,
    include_system_fonts: bool,
    include_embedded_fonts: bool,
) -> Vec<Font> {
    // typst-kit returns fallible font sources. Unloadable files are skipped to
    // match Typst CLI behavior and to tolerate non-font files in user folders.
    let mut fonts: Vec<Font> = font_dirs
        .iter()
        .flat_map(|dir| scan(dir))
        .filter_map(|(source, _)| source.load())
        .collect();

    if include_system_fonts {
        fonts.extend(system().filter_map(|(source, _)| source.load()));
    }

    if include_embedded_fonts {
        fonts.extend(embedded().map(|(font, _)| font));
    }

    fonts
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
