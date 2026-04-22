use crate::multipage::{render_page_template_path, validate_multipage_template};
use crate::output::OutputFormat;

use std::path::{Path, PathBuf};
use typst::layout::PagedDocument;
use typst_html::HtmlDocument;
use typst_pdf::{PdfOptions, PdfStandards};

pub fn write_pdf(
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

pub fn write_html(document: &HtmlDocument, output_path: &Path) -> std::result::Result<(), String> {
    let html: String =
        typst_html::html(document).map_err(|err| format!("HTML export failed: {err:?}"))?;

    std::fs::write(output_path, html.as_bytes())
        .map_err(|err| format!("Could not write HTML to {}: {err}", output_path.display()))
}

pub fn write_png(
    document: &PagedDocument,
    output_path: &Path,
    ppi: &f32,
) -> std::result::Result<(), String> {
    let total_pages: usize = document.pages.len();
    validate_multipage_template(output_path, total_pages, OutputFormat::Png)?;

    for (index, page) in document.pages.iter().enumerate() {
        let page_number: usize = index + 1;
        let page_output_path: PathBuf = if total_pages > 1 {
            render_page_template_path(output_path, page_number, total_pages)
        } else {
            output_path.to_path_buf()
        };
        let pixmap = typst_render::render(page, ppi / 72.0);
        let png: Vec<u8> = pixmap
            .encode_png()
            .map_err(|err| format!("PNG export failed: {err}"))?;

        std::fs::write(&page_output_path, png).map_err(|err| {
            format!(
                "Could not write PNG to {}: {err}",
                page_output_path.display()
            )
        })?;
    }

    Ok(())
}

pub fn write_svg(document: &PagedDocument, output_path: &Path) -> std::result::Result<(), String> {
    let total_pages: usize = document.pages.len();
    validate_multipage_template(output_path, total_pages, OutputFormat::Svg)?;

    for (index, page) in document.pages.iter().enumerate() {
        let page_number: usize = index + 1;
        let page_output_path: PathBuf = if total_pages > 1 {
            render_page_template_path(output_path, page_number, total_pages)
        } else {
            output_path.to_path_buf()
        };
        let svg: String = typst_svg::svg(page);

        std::fs::write(&page_output_path, svg.as_bytes()).map_err(|err| {
            format!(
                "Could not write SVG to {}: {err}",
                page_output_path.display()
            )
        })?;
    }

    Ok(())
}
