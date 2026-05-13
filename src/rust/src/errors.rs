use std::fmt::Write as _;
use std::path::Path;

use typst::diag::{HintedString, Severity, SourceDiagnostic};
use typst::syntax::{FileId, Source};
use typst_as_lib::TypstAsLibError;

pub fn format_typst_errors(root: &Path, err: &TypstAsLibError) -> String {
    match err {
        TypstAsLibError::TypstSource(errors) => errors
            .iter()
            .map(|diag| format_source_diagnostic(root, diag))
            .collect::<Vec<_>>()
            .join("\n\n"),
        TypstAsLibError::HintedString(hinted) => format_hinted_string(hinted),
        TypstAsLibError::TypstFile(file_error) => format!("error: {file_error}"),
        other => format!("error: {other}"),
    }
}

pub fn format_typst_warnings(root: &Path, warnings: &[SourceDiagnostic]) -> Vec<String> {
    warnings
        .iter()
        .map(|d| format_source_diagnostic(root, d))
        .collect()
}

fn format_hinted_string(hinted: &HintedString) -> String {
    let mut out = format!("error: {}", hinted.message());
    for hint in hinted.hints() {
        let _ = write!(out, "\nhint: {hint}");
    }
    out
}

fn format_source_diagnostic(root: &Path, diagnostic: &SourceDiagnostic) -> String {
    let mut out = format!(
        "{}: {}",
        severity_label(diagnostic.severity),
        diagnostic.message
    );

    let Some(file_id) = diagnostic.span.id() else {
        append_hints(&mut out, diagnostic);
        return out;
    };

    let Ok(source) = load_source(root, file_id) else {
        append_hints(&mut out, diagnostic);
        return out;
    };

    let Some(range) = source.range(diagnostic.span) else {
        append_hints(&mut out, diagnostic);
        return out;
    };

    let lines = source.lines();
    let line_idx = lines.byte_to_line(range.start).unwrap_or(0);
    let col_idx = lines.byte_to_column(range.start).unwrap_or(0);
    let line_range = lines.line_to_range(line_idx).unwrap();
    let line_no = line_idx + 1;
    let raw_line = &source.text()[line_range];
    let line_text = raw_line.trim_end_matches(['\n', '\r']);

    let end_col = if range.start < range.end
        && lines.byte_to_line(range.end.saturating_sub(1)) == Some(line_idx)
    {
        lines.byte_to_column(range.end).unwrap_or(col_idx + 1)
    } else {
        line_text.chars().count()
    };

    let caret_count = end_col.saturating_sub(col_idx).max(1);
    let gutter = line_no.to_string().len();

    let _ = write!(
        out,
        "\n  ┌─ {}:{}:{}\n  │\n{line_no:>gutter$} │ {line_text}\n  │ {}{}",
        display_file_id(file_id),
        line_no,
        col_idx + 1,
        " ".repeat(col_idx),
        "^".repeat(caret_count),
    );

    append_hints(&mut out, diagnostic);
    out
}

fn append_hints(out: &mut String, diagnostic: &SourceDiagnostic) {
    for hint in &diagnostic.hints {
        let _ = write!(out, "\nhint: {hint}");
    }
}

fn load_source(root: &Path, file_id: FileId) -> Result<Source, String> {
    let path = match file_id.package() {
        None => file_id.vpath().resolve(root),
        Some(_) => None,
    }
    .ok_or_else(|| "could not resolve source path".to_string())?;

    let text = std::fs::read_to_string(&path)
        .map_err(|err| format!("could not read {}: {err}", path.display()))?;

    Ok(Source::new(file_id, text))
}

fn display_file_id(file_id: FileId) -> String {
    match file_id.package() {
        None => file_id.vpath().as_rootless_path().display().to_string(),
        Some(package) => format!("{package}{}", file_id.vpath().as_rooted_path().display()),
    }
}

fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    }
}
