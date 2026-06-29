use std::fmt::Write as _;
use std::path::Path;

use typst::diag::{Severity, SourceDiagnostic};
use typst::syntax::{FileId, VirtualRoot};
use typst::{World, WorldExt};

pub fn format_typst_errors(world: &dyn World, root: &Path, errors: &[SourceDiagnostic]) -> String {
    errors
        .iter()
        .map(|diag| format_source_diagnostic(world, root, diag))
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub fn format_typst_warnings(
    world: &dyn World,
    root: &Path,
    warnings: &[SourceDiagnostic],
) -> Vec<String> {
    warnings
        .iter()
        .map(|d| format_source_diagnostic(world, root, d))
        .collect()
}

fn format_source_diagnostic(
    world: &dyn World,
    root: &Path,
    diagnostic: &SourceDiagnostic,
) -> String {
    let mut out = format!(
        "{}: {}",
        severity_label(diagnostic.severity),
        diagnostic.message
    );

    let Some(file_id) = diagnostic.span.id() else {
        append_hints(&mut out, diagnostic);
        return out;
    };

    let Ok(source) = world.source(file_id) else {
        append_hints(&mut out, diagnostic);
        return out;
    };

    let Some(range) = world.range(diagnostic.span) else {
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
        display_file_id(root, file_id),
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
        let _ = write!(out, "\nhint: {}", hint.v);
    }
}

fn display_file_id(root: &Path, file_id: FileId) -> String {
    match file_id.root() {
        VirtualRoot::Project => file_id
            .vpath()
            .realize(root)
            .ok()
            .and_then(|path| {
                path.strip_prefix(root)
                    .ok()
                    .map(|path| path.display().to_string())
            })
            .unwrap_or_else(|| file_id.vpath().get_without_slash().to_owned()),
        VirtualRoot::Package(package) => {
            format!("{package}{}", file_id.vpath().get_with_slash())
        }
    }
}

fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    }
}
