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
        // Some diagnostics are not tied to a source span, for example errors
        // raised while resolving packages or validating document-level options.
        append_hints(&mut out, diagnostic);
        return out;
    };

    let Ok(source) = world.source(file_id) else {
        // If the source can no longer be read, keep the primary Typst message
        // instead of replacing it with an internal formatting failure.
        append_hints(&mut out, diagnostic);
        return out;
    };

    let Some(range) = world.range(diagnostic.span) else {
        // Typst can emit synthetic spans that do not map cleanly back to a byte
        // range. The message and hints are still useful to R callers.
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
        // Project files are shown relative to the user-selected root so errors
        // are stable and readable even when the temp directory changes.
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
            // Package files do not live under the project root, so keep Typst's
            // package identity in the display path.
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
