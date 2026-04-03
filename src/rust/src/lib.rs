mod fonts;
mod inputs;
mod output;
mod standard;

use extendr_api::prelude::*;
use fonts::load_fonts_from_dir;
use inputs::build_sys_inputs;
use output::{infer_output_format, OutputFormat};
use standard::parse_pdf_standards;
use std::path::{Path, PathBuf};
use typst::foundations::Dict;
use typst::layout::{Abs, PagedDocument};
use typst::visualize::Color;
use typst_as_lib::TypstEngine;
use typst_html::HtmlDocument;
use typst_pdf::{PdfOptions, PdfStandards};

fn build_engine(root: &Path, font_path: Option<&str>) -> std::result::Result<TypstEngine, String> {
    match font_path {
        Some(path) => {
            let fonts: Vec<Vec<u8>> = load_fonts_from_dir(path)?;
            Ok(TypstEngine::builder()
                .with_file_system_resolver(root)
                .fonts(fonts)
                .build())
        }
        None => {
            let fonts = typst_assets::fonts();
            Ok(TypstEngine::builder()
                .with_file_system_resolver(root)
                .fonts(fonts)
                .build())
        }
    }
}

fn compile_paged_document(
    engine: &TypstEngine,
    main_file: &str,
    sys_inputs: &Dict,
) -> std::result::Result<PagedDocument, String> {
    engine
        .compile_with_input(main_file, sys_inputs.clone())
        .output
        .map_err(|err| format!("Typst compilation failed: {err}"))
}

fn compile_html_document(
    engine: &TypstEngine,
    main_file: &str,
    sys_inputs: &Dict,
) -> std::result::Result<HtmlDocument, String> {
    engine
        .compile_with_input(main_file, sys_inputs.clone())
        .output
        .map_err(|err| format!("Typst compilation failed: {err}"))
}

fn write_pdf(
    document: &PagedDocument,
    output_path: &Path,
    standards: PdfStandards,
) -> std::result::Result<(), String> {
    let pdf_options: PdfOptions<'_> = PdfOptions {
        standards,
        ..Default::default()
    };

    let pdf: Vec<u8> = typst_pdf::pdf(document, &pdf_options)
        .map_err(|err| format!("PDF export failed: {err:?}"))?;

    std::fs::write(output_path, pdf)
        .map_err(|err| format!("Could not write PDF to {}: {err}", output_path.display()))
}

fn write_html(document: &HtmlDocument, output_path: &Path) -> std::result::Result<(), String> {
    let html: String =
        typst_html::html(document).map_err(|err| format!("HTML export failed: {err:?}"))?;

    std::fs::write(output_path, html.as_bytes())
        .map_err(|err| format!("Could not write HTML to {}: {err}", output_path.display()))
}

fn write_png(document: &PagedDocument, output_path: &Path) -> std::result::Result<(), String> {
    // Keep the public API returning one output path by vertically merging pages.
    let pixmap =
        typst_render::render_merged(document, 144.0 / 72.0, Abs::pt(1.0), Some(Color::WHITE));
    let png: Vec<u8> = pixmap
        .encode_png()
        .map_err(|err| format!("PNG export failed: {err}"))?;

    std::fs::write(output_path, png)
        .map_err(|err| format!("Could not write PNG to {}: {err}", output_path.display()))
}

fn write_svg(document: &PagedDocument, output_path: &Path) -> std::result::Result<(), String> {
    // Keep the public API returning one output path by vertically merging pages.
    let svg: String = typst_svg::svg_merged(document, Abs::pt(1.0));

    std::fs::write(output_path, svg.as_bytes())
        .map_err(|err| format!("Could not write SVG to {}: {err}", output_path.display()))
}

/// Compiles a `.typ` Typst file into a supported output format.
///
/// The function loads the specified Typst file, compiles it using a `TypstEngine`,
/// and writes the resulting output to disk. Fonts can optionally be loaded from a
/// custom directory. PDF output can additionally enforce a specific PDF standard.
///
/// # Arguments
///
/// * `file` - Path to the input `.typ` file to compile.
/// * `output` - Optional path where the generated file will be written. If `None`,
///   the output path defaults to the input file with the extension implied by the
///   effective output format.
/// * `font_path` - Optional directory containing fonts to load for the Typst engine.
///   If `None`, the default fonts from `typst_assets` are used.
/// * `pdf_standard` - Optional string describing the PDF standard(s) to enforce
///   (for example `pdf/a-2b`). This is only supported for PDF output.
/// * `output_format` - Optional output format. Supported values are `pdf`, `html`,
///   `png`, and `svg`. If `None`, the format is inferred from the output path when
///   possible and otherwise defaults to `pdf`.
/// * `root` - Optional Typst project root. If `None`, it defaults to the parent
///   directory of the main Typst file. When provided, the main file must be
///   contained in the root directory's subtree.
///
/// # Returns
///
/// Returns `Ok(String)` containing the path to the generated file on success.
///
/// # Errors
///
/// Returns `Err(String)` if:
///
/// * The input file does not exist or is not a `.typ` file.
/// * The input file name is not valid UTF-8.
/// * The output path, PDF standard, or output format argument is empty.
/// * Font loading fails.
/// * Typst compilation fails.
/// * Export generation fails.
/// * The generated file cannot be written to disk.
///
/// # Behavior
///
/// By default, the Typst project root is set to the parent directory of the
/// input file. When `root` is provided, Typst resolves absolute paths from that
/// directory and the main file must still live within its subtree. Multi-page
/// PNG and SVG exports are merged into a single vertically stacked image so the
/// function can keep returning a single output path.
fn compile_file(
    file: &str,
    output: Option<&str>,
    font_path: Option<&str>,
    pdf_standard: Option<&str>,
    output_format: Option<&str>,
    root: Option<&str>,
    inputs: Option<&[String]>,
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

    let canonical_input_path: PathBuf = std::fs::canonicalize(input_path).map_err(|err| {
        format!(
            "Could not resolve input file {}: {err}",
            input_path.display()
        )
    })?;

    let root_path: PathBuf = match root {
        Some(root) if root.trim().is_empty() => {
            return Err("`root` must not be an empty path".to_owned());
        }
        Some(root) => {
            let root_path: &Path = Path::new(root);
            if !root_path.is_dir() {
                return Err(format!(
                    "Root directory does not exist: {}",
                    root_path.display()
                ));
            }
            std::fs::canonicalize(root_path).map_err(|err| {
                format!(
                    "Could not resolve root directory {}: {err}",
                    root_path.display()
                )
            })?
        }
        None => canonical_input_path
            .parent()
            .ok_or_else(|| {
                format!(
                    "Could not determine the parent directory of input file: {}",
                    input_path.display()
                )
            })?
            .to_path_buf(),
    };

    let main_file: String = canonical_input_path
        .strip_prefix(&root_path)
        .map_err(|_| {
            format!(
                "Input file must be contained in the root directory: {} (root: {})",
                input_path.display(),
                root_path.display()
            )
        })?
        .to_string_lossy()
        .into_owned();

    let explicit_output_path: Option<PathBuf> = match output {
        Some(path) if path.trim().is_empty() => {
            return Err("`output` must not be an empty path".to_owned());
        }
        Some(path) => Some(PathBuf::from(path)),
        None => None,
    };

    let output_format: OutputFormat =
        infer_output_format(explicit_output_path.as_deref(), output_format)?;
    let output_path: PathBuf = explicit_output_path
        .unwrap_or_else(|| input_path.with_extension(output_format.extension()));

    let standards: PdfStandards = if output_format == OutputFormat::Pdf {
        match pdf_standard {
            Some(value) if value.trim().is_empty() => {
                return Err("`pdf_standard` must not be empty".to_owned());
            }
            Some(value) => parse_pdf_standards(value)?,
            None => PdfStandards::default(),
        }
    } else {
        if pdf_standard.is_some() {
            return Err(
                "`pdf_standard` is only supported when `output_format` is `pdf`".to_owned(),
            );
        }
        PdfStandards::default()
    };

    let engine: TypstEngine = build_engine(&root_path, font_path)?;
    let sys_inputs: Dict = build_sys_inputs(inputs)?;

    match output_format {
        OutputFormat::Pdf => {
            let doc: PagedDocument = compile_paged_document(&engine, &main_file, &sys_inputs)?;
            write_pdf(&doc, &output_path, standards)?;
        }
        OutputFormat::Html => {
            let doc: HtmlDocument = compile_html_document(&engine, &main_file, &sys_inputs)?;
            write_html(&doc, &output_path)?;
        }
        OutputFormat::Png => {
            let doc: PagedDocument = compile_paged_document(&engine, &main_file, &sys_inputs)?;
            write_png(&doc, &output_path)?;
        }
        OutputFormat::Svg => {
            let doc: PagedDocument = compile_paged_document(&engine, &main_file, &sys_inputs)?;
            write_svg(&doc, &output_path)?;
        }
    }

    Ok(output_path.to_string_lossy().into_owned())
}

/// @title Compile a `.typ` file and return the output path.
///
/// @description This function uses the Typst Rust library to compile a
/// `.typ` file to a supported output format and return the output path.
///
/// @param file Path to an existing `.typ` file.
/// @param output Optional output path.
/// @param font_path Optional path to font files.
/// @param pdf_standard Optional PDF standard specification.
/// @param output_format Optional output format.
/// @param root Optional Typst project root. If `None`, it defaults to the parent
///   directory of `file`. When provided, `file` must be contained in the root
///   directory's subtree.
///
/// @return Output path
///
/// @keywords internal
#[extendr]
fn typst_compile_rust(
    file: &str,
    #[default = "NULL"] output: Nullable<String>,
    #[default = "NULL"] font_path: Nullable<String>,
    #[default = "NULL"] pdf_standard: Nullable<String>,
    #[default = "NULL"] output_format: Nullable<String>,
    #[default = "NULL"] root: Nullable<String>,
    #[default = "NULL"] inputs: Nullable<Vec<String>>,
) -> String {
    let output: Option<String> = output.into_option();
    let font_path: Option<String> = font_path.into_option();
    let pdf_standard: Option<String> = pdf_standard.into_option();
    let output_format: Option<String> = output_format.into_option();
    let root: Option<String> = root.into_option();
    let inputs: Option<Vec<String>> = inputs.into_option();

    match compile_file(
        file,
        output.as_deref(),
        font_path.as_deref(),
        pdf_standard.as_deref(),
        output_format.as_deref(),
        root.as_deref(),
        inputs.as_deref(),
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
    fn typst_compile_rust;
}

#[cfg(test)]
mod tests {
    use super::{compile_file, load_fonts_from_dir};
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

    fn assert_is_pdf(path: &Path) {
        let bytes: Vec<u8> = fs::read(path).expect("could not read generated PDF");
        assert!(bytes.starts_with(b"%PDF"), "generated file is not a PDF");
    }

    fn assert_is_html(path: &Path) {
        let html: String = fs::read_to_string(path).expect("could not read generated HTML");
        assert!(
            html.contains("<!DOCTYPE html>"),
            "generated file is not an HTML document"
        );
    }

    fn assert_is_png(path: &Path) {
        let bytes: Vec<u8> = fs::read(path).expect("could not read generated PNG");
        assert!(
            bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
            "generated file is not a PNG"
        );
    }

    fn assert_is_svg(path: &Path) {
        let svg: String = fs::read_to_string(path).expect("could not read generated SVG");
        assert!(svg.contains("<svg"), "generated file is not an SVG");
    }

    fn fixture_font_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/testthat/fonts")
    }

    fn copy_fixture_fonts(destination: &Path) {
        let source_dir: PathBuf = fixture_font_dir();
        assert!(
            source_dir.is_dir(),
            "fixture font directory does not exist: {}",
            source_dir.display()
        );

        for file_name in ["Amarante-Regular.ttf", "Ultra-Regular.ttf"] {
            let source: PathBuf = source_dir.join(file_name);
            let destination_path: PathBuf = destination.join(file_name);
            fs::copy(&source, &destination_path).unwrap_or_else(|err| {
                panic!(
                    "could not copy font fixture from {} to {}: {err}",
                    source.display(),
                    destination_path.display()
                )
            });
        }
    }

    #[test]
    fn compile_uses_default_pdf_path_when_output_is_none() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("default.typ");
        let expected_pdf: PathBuf = dir.join("default.pdf");
        write_typ_file(&typ_path, "= Hello from test");

        let output: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
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
        assert_is_pdf(&expected_pdf);

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_writes_pdf_to_custom_output_path() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("source.typ");
        let custom_pdf: PathBuf = dir.join("custom-output.pdf");
        write_typ_file(&typ_path, "= Hello custom output");

        let output: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            Some(custom_pdf.to_str().expect("path should be valid UTF-8")),
            None,
            None,
            None,
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
        assert_is_pdf(&custom_pdf);

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn load_fonts_from_dir_reads_fixture_fonts_and_skips_other_entries() {
        let dir: PathBuf = unique_temp_dir();
        let font_dir: PathBuf = dir.join("fonts");
        fs::create_dir_all(font_dir.join("nested"))
            .expect("could not create nested font directory");
        copy_fixture_fonts(&font_dir);
        fs::write(font_dir.join("README.txt"), "not a font")
            .expect("could not write non-font file");

        let fonts: Vec<Vec<u8>> =
            load_fonts_from_dir(font_dir.to_str().expect("path should be valid UTF-8"))
                .expect("font loading should succeed");

        let mut loaded_sizes: Vec<usize> = fonts.iter().map(Vec::len).collect();
        loaded_sizes.sort_unstable();

        let mut expected_sizes: Vec<usize> = ["Amarante-Regular.ttf", "Ultra-Regular.ttf"]
            .into_iter()
            .map(|file_name| {
                fs::metadata(font_dir.join(file_name))
                    .expect("fixture font should exist")
                    .len() as usize
            })
            .collect();
        expected_sizes.sort_unstable();

        assert_eq!(loaded_sizes, expected_sizes);

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_succeeds_with_custom_fixture_fonts() {
        let dir: PathBuf = unique_temp_dir();
        let font_dir: PathBuf = dir.join("fonts");
        let typ_path: PathBuf = dir.join("custom-font.typ");
        let expected_pdf: PathBuf = dir.join("custom-font.pdf");
        fs::create_dir_all(&font_dir).expect("could not create font directory");
        copy_fixture_fonts(&font_dir);
        write_typ_file(
            &typ_path,
            "#set document(title: \"Fixture fonts\")\n#set text(font: \"Ultra\")\n= Hello from custom font",
        );

        let output: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            Some(font_dir.to_str().expect("path should be valid UTF-8")),
            None,
            None,
            None,
            None,
        )
        .expect("compilation with custom fonts should succeed");

        assert_eq!(PathBuf::from(output), expected_pdf);
        assert!(expected_pdf.exists(), "expected PDF output to exist");
        assert_is_pdf(&expected_pdf);

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_succeeds_for_nested_main_file_with_explicit_root() {
        let dir: PathBuf = unique_temp_dir();
        let root_dir: PathBuf = dir.join("project");
        let nested_dir: PathBuf = root_dir.join("subdir");
        let typ_path: PathBuf = nested_dir.join("nested.typ");
        let expected_pdf: PathBuf = nested_dir.join("nested.pdf");
        fs::create_dir_all(&nested_dir).expect("could not create nested project directory");
        write_typ_file(&typ_path, "= Hello from nested root test");

        let output: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
            None,
            Some(root_dir.to_str().expect("path should be valid UTF-8")),
            None,
        )
        .expect("compilation with an explicit project root should succeed");

        assert_eq!(PathBuf::from(output), expected_pdf);
        assert!(expected_pdf.exists(), "expected PDF output to exist");
        assert_is_pdf(&expected_pdf);

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_succeeds_with_supported_pdf_standard() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("pdf-standard.typ");
        let expected_pdf: PathBuf = dir.join("pdf-standard.pdf");
        write_typ_file(
            &typ_path,
            "#set document(title: \"PDF standard test\")\n= Hello from PDF standard test",
        );

        let output: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            Some("1.7"),
            None,
            None,
            None,
        )
        .expect("compilation with a supported PDF standard should succeed");

        assert_eq!(PathBuf::from(output), expected_pdf);
        assert!(expected_pdf.exists(), "expected PDF output to exist");
        assert_is_pdf(&expected_pdf);

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_for_missing_input_file() {
        let dir: PathBuf = unique_temp_dir();
        let missing: PathBuf = dir.join("missing.typ");

        let err: String = compile_file(
            missing.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .expect_err("missing file should return an error");

        assert!(err.contains("Input file does not exist"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_when_input_is_outside_explicit_root() {
        let dir: PathBuf = unique_temp_dir();
        let root_dir: PathBuf = dir.join("project");
        let outside_dir: PathBuf = dir.join("outside");
        let typ_path: PathBuf = outside_dir.join("source.typ");
        fs::create_dir_all(&root_dir).expect("could not create project root directory");
        fs::create_dir_all(&outside_dir).expect("could not create outside directory");
        write_typ_file(&typ_path, "= Outside root");

        let err: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
            None,
            Some(root_dir.to_str().expect("path should be valid UTF-8")),
            None,
        )
        .expect_err("input outside the explicit root should return an error");

        assert!(err.contains("Input file must be contained in the root directory"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_for_non_typ_extension() {
        let dir: PathBuf = unique_temp_dir();
        let txt_path: PathBuf = dir.join("source.txt");
        write_typ_file(&txt_path, "= wrong extension");

        let err: String = compile_file(
            txt_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
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

        let err: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            Some(""),
            None,
            None,
            None,
            None,
            None,
        )
        .expect_err("empty output path should return an error");

        assert!(err.contains("`output` must not be an empty path"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_for_empty_pdf_standard() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("source.typ");
        write_typ_file(&typ_path, "= Empty PDF standard");

        let err: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            Some(""),
            None,
            None,
            None,
        )
        .expect_err("empty PDF standard should return an error");

        assert!(err.contains("`pdf_standard` must not be empty"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_for_unsupported_pdf_standard() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("source.typ");
        write_typ_file(&typ_path, "= Unsupported PDF standard");

        let err: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            Some("bogus-standard"),
            None,
            None,
            None,
        )
        .expect_err("unsupported PDF standard should return an error");

        assert!(err.contains("Unsupported PDF standard: bogus-standard"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_for_unsupported_pdf_ua_2_standard() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("source.typ");
        write_typ_file(&typ_path, "= Unsupported PDF/UA standard");

        let err: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            Some("ua-2"),
            None,
            None,
            None,
        )
        .expect_err("unsupported PDF/UA-2 standard should return an error");

        assert!(err.contains("does not support PDF/UA-2"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_uses_default_html_path_when_output_format_is_html() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("default-html.typ");
        let expected_html: PathBuf = dir.join("default-html.html");
        write_typ_file(&typ_path, "= Hello HTML");

        let output: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
            Some("html"),
            None,
            None,
        )
        .expect("HTML compilation should succeed");

        assert_eq!(PathBuf::from(output), expected_html);
        assert!(expected_html.exists(), "expected HTML output to exist");
        assert_is_html(&expected_html);

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_infers_html_output_format_from_output_extension() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("source.typ");
        let custom_html: PathBuf = dir.join("custom-output.html");
        write_typ_file(&typ_path, "= Hello inferred HTML");

        let output: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            Some(custom_html.to_str().expect("path should be valid UTF-8")),
            None,
            None,
            None,
            None,
            None,
        )
        .expect("compilation should infer HTML from the output extension");

        assert_eq!(PathBuf::from(output), custom_html);
        assert!(custom_html.exists(), "expected HTML output to exist");
        assert_is_html(&custom_html);

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_writes_merged_png_output() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("merged-png.typ");
        let expected_png: PathBuf = dir.join("merged-png.png");
        write_typ_file(&typ_path, "= First page\n#pagebreak()\n= Second page");

        let output: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
            Some("png"),
            None,
            None,
        )
        .expect("PNG compilation should succeed");

        assert_eq!(PathBuf::from(output), expected_png);
        assert!(expected_png.exists(), "expected PNG output to exist");
        assert_is_png(&expected_png);

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_writes_merged_svg_output() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("merged-svg.typ");
        let expected_svg: PathBuf = dir.join("merged-svg.svg");
        write_typ_file(&typ_path, "= First page\n#pagebreak()\n= Second page");

        let output: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
            Some("svg"),
            None,
            None,
        )
        .expect("SVG compilation should succeed");

        assert_eq!(PathBuf::from(output), expected_svg);
        assert!(expected_svg.exists(), "expected SVG output to exist");
        assert_is_svg(&expected_svg);

        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_for_empty_output_format() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("source.typ");
        write_typ_file(&typ_path, "= Empty output format");

        let err: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            None,
            Some(""),
            None,
            None,
        )
        .expect_err("empty output format should return an error");

        assert!(err.contains("`output_format` must not be empty"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_for_unknown_output_extension_without_output_format() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("source.typ");
        write_typ_file(&typ_path, "= Unknown output extension");

        let err: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            Some(
                dir.join("output.weird")
                    .to_str()
                    .expect("path should be valid UTF-8"),
            ),
            None,
            None,
            None,
            None,
            None,
        )
        .expect_err("unknown output extension should return an error");

        assert!(err.contains("Could not infer output format from output path"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }

    #[test]
    fn compile_fails_when_pdf_standard_is_used_for_non_pdf_output() {
        let dir: PathBuf = unique_temp_dir();
        let typ_path: PathBuf = dir.join("source.typ");
        write_typ_file(&typ_path, "= Non PDF output");

        let err: String = compile_file(
            typ_path.to_str().expect("path should be valid UTF-8"),
            None,
            None,
            Some("1.7"),
            Some("html"),
            None,
            None,
        )
        .expect_err("pdf_standard should be rejected for non-PDF output");

        assert!(err.contains("`pdf_standard` is only supported when `output_format` is `pdf`"));
        fs::remove_dir_all(dir).expect("could not remove temp directory");
    }
}
