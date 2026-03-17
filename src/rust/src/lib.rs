mod fonts;
mod standard;

use extendr_api::prelude::*;
use fonts::load_fonts_from_dir;
use standard::parse_pdf_standards;
use std::path::{Path, PathBuf};
use typst::layout::PagedDocument;
use typst_as_lib::TypstEngine;
use typst_pdf::{PdfOptions, PdfStandards};

/// Compiles a `.typ` Typst file into a PDF document.
///
/// The function loads the specified Typst file, compiles it using a `TypstEngine`,
/// and writes the resulting PDF to disk. Fonts can optionally be loaded from a
/// custom directory, and a specific PDF standard can be enforced.
///
/// # Arguments
///
/// * `file` - Path to the input `.typ` file to compile.
/// * `output` - Optional path where the generated PDF will be written.  
///   If `None`, the output path defaults to the input file with the `.pdf` extension.
/// * `font_path` - Optional directory containing fonts to load for the Typst engine.
///   If `None`, the default fonts from `typst_assets` are used.
/// * `pdf_standard` - Optional string describing the PDF standard(s) to enforce
///   (for example `pdf/a-2b`). If `None`, the default `PdfStandards` configuration is used.
///
/// # Returns
///
/// Returns `Ok(String)` containing the path to the generated PDF on success.
///
/// # Errors
///
/// Returns `Err(String)` if:
///
/// * The input file does not exist or is not a `.typ` file.
/// * The input file name is not valid UTF-8.
/// * The output path or PDF standard argument is empty.
/// * Font loading fails.
/// * Typst compilation fails.
/// * PDF generation fails.
/// * The PDF cannot be written to disk.
///
/// # Behavior
///
/// The Typst project root is set to the parent directory of the input file.
/// The file name itself is passed to the Typst compiler, allowing relative
/// imports within the same project directory.
fn compile_file_to_pdf(
    file: &str,
    output: Option<&str>,
    font_path: Option<&str>,
    pdf_standard: Option<&str>,
) -> std::result::Result<String, String> {
    let input_path: &Path = Path::new(file);
    if !input_path.is_file() {
        return Err(format!(
            "Input file does not exist: {}",
            input_path.display()
        ));
    }

    match input_path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("typ") => {}
        _ => return Err(format!("Input file must have a .typ extension: {}", file)),
    }

    let root: &Path = input_path.parent().unwrap_or_else(|| Path::new("."));
    let main_file: &str = input_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("Invalid UTF-8 file name: {}", file))?;
    let output_path: PathBuf = match output {
        Some(path) if path.trim().is_empty() => {
            return Err("`output` must not be an empty path".to_owned());
        }
        Some(path) => PathBuf::from(path),
        None => input_path.with_extension("pdf"),
    };

    let standards: PdfStandards = match pdf_standard {
        Some(value) if value.trim().is_empty() => {
            return Err("`pdf_standard` must not be empty".to_owned());
        }
        Some(value) => parse_pdf_standards(value)?,
        None => PdfStandards::default(),
    };

    let engine: TypstEngine = match font_path {
        Some(path) => {
            let fonts: Vec<Vec<u8>> = load_fonts_from_dir(path)?;
            TypstEngine::builder()
                .with_file_system_resolver(root)
                .fonts(fonts)
                .build()
        }
        None => {
            let fonts = typst_assets::fonts();
            TypstEngine::builder()
                .with_file_system_resolver(root)
                .fonts(fonts)
                .build()
        }
    };

    let doc: PagedDocument = engine
        .compile(main_file)
        .output
        .map_err(|err| format!("Typst compilation failed: {err}"))?;

    let pdf_options = PdfOptions {
        standards,
        ..Default::default()
    };

    let pdf: Vec<u8> =
        typst_pdf::pdf(&doc, &pdf_options).map_err(|err| format!("PDF export failed: {err:?}"))?;

    std::fs::write(&output_path, pdf)
        .map_err(|err| format!("Could not write PDF to {}: {err}", output_path.display()))?;

    Ok(output_path.to_string_lossy().into_owned())
}

/// @title Compile a `.typ` file to a `.pdf` file and return the output path.
///
/// @description This functions uses the Tyspt Rust library to compile a
/// `.typ` file to a `.pdf` file and return the output path.
///
/// @param file Path to an existing `.typ` file.
/// @param output Optional output path. Defaults to the input path with `.pdf`.
/// @param pdf_standard Optional PDF standard specification. Options are: : `1.4`,
/// `1.5`, `1.6`, `1.7`, `2.0`, `a-1b`, `a-1a`, `a-2b`, `a-2u`, `a-2a`, `a-3b`,
/// `a-3u`, `a-3a`, `a-4`, `a-4f`, `a-4e`, `ua-1`. Default to `NULL`.
///
/// @return Output path, invisibly.
///
/// @export
#[extendr]
fn typst_compile(
    file: &str,
    #[default = "NULL"] output: Nullable<String>,
    #[default = "NULL"] font_path: Nullable<String>,
    #[default = "NULL"] pdf_standard: Nullable<String>,
) -> String {
    let output: Option<String> = output.into_option();
    let font_path: Option<String> = font_path.into_option();
    let pdf_standard: Option<String> = pdf_standard.into_option();

    match compile_file_to_pdf(
        file,
        output.as_deref(),
        font_path.as_deref(),
        pdf_standard.as_deref(),
    ) {
        Ok(output_path) => output_path,
        Err(message) => throw_r_error(message),
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod tynding;
    fn typst_compile;
}

#[cfg(test)]
mod tests {
    use super::compile_file_to_pdf;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir() -> PathBuf {
        let nanos: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        let dir: PathBuf =
            std::env::temp_dir().join(format!("tynding-tests-{}-{}", std::process::id(), nanos));
        fs::create_dir_all(&dir).expect("could not create temp directory");
        dir
    }

    fn write_typ_file(path: &Path, content: &str) {
        fs::write(path, content).expect("could not write typst source file");
    }

    #[test]
    fn compile_uses_default_pdf_path_when_output_is_none() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("default.typ");
        let expected_pdf: PathBuf = dir.join("default.pdf");
        write_typ_file(&typ_path, "= Hello from test");

        let output: String = compile_file_to_pdf(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
        )
        .expect("compilation should succeed");

        let output_path: PathBuf = PathBuf::from(output);
        assert_eq!(output_path, expected_pdf);
        assert!(
            expected_pdf.exists(),
            "expected default output PDF to exist"
        );

        let bytes: Vec<u8> = fs::read(&expected_pdf).expect("could not read generated PDF");
        assert!(bytes.starts_with(b"%PDF"), "generated file is not a PDF");

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_writes_pdf_to_custom_output_path() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("source.typ");
        let custom_pdf: PathBuf = dir.join("custom-output.pdf");
        write_typ_file(&typ_path, "= Hello custom output");

        let output: String = compile_file_to_pdf(
            typ_path.to_str().expect("path should be valid UTF-8"),
            Some(custom_pdf.to_str().expect("path should be valid UTF-8")),
            None,
            None,
        )
        .expect("compilation should succeed");

        let output_path: PathBuf = PathBuf::from(output);
        assert_eq!(output_path, custom_pdf);
        assert!(custom_pdf.exists(), "expected custom output PDF to exist");
        assert!(
            !dir.join("source.pdf").exists(),
            "default output path should not be used when custom output is provided"
        );

        let bytes: Vec<u8> = fs::read(&custom_pdf).expect("could not read generated PDF");
        assert!(bytes.starts_with(b"%PDF"), "generated file is not a PDF");

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_for_missing_input_file() {
        let dir: PathBuf = unique_temp_dir();
        let missing: PathBuf = dir.join("missing.typ");

        let err: String = compile_file_to_pdf(
            missing.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
        )
        .expect_err("missing file should return an error");

        assert!(err.contains("Input file does not exist"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_for_non_typ_extension() {
        let dir: PathBuf = unique_temp_dir();
        let txt_path: PathBuf = dir.join("source.txt");
        write_typ_file(&txt_path, "= wrong extension");

        let err: String = compile_file_to_pdf(
            txt_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
        )
        .expect_err("non-.typ input should return an error");

        assert!(err.contains("must have a .typ extension"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_for_empty_output_path() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("source.typ");
        write_typ_file(&typ_path, "= Empty output path");

        let err: String = compile_file_to_pdf(
            typ_path.to_str().expect("path should be valid UTF-8"),
            Some(""),
            None,
            None,
        )
        .expect_err("empty output path should return an error");

        assert!(err.contains("`output` must not be an empty path"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }
}
