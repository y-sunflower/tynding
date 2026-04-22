use crate::output::OutputFormat;
use std::path::{Path, PathBuf};

pub fn render_page_template_path(template: &Path, page: usize, total_pages: usize) -> PathBuf {
    let width: usize = total_pages.to_string().len();
    let rendered: String = template
        .to_string_lossy()
        .replace("{0p}", &format!("{page:0width$}"))
        .replace("{p}", &page.to_string())
        .replace("{t}", &total_pages.to_string());
    PathBuf::from(rendered)
}

pub fn validate_multipage_template(
    output_path: &Path,
    total_pages: usize,
    format: OutputFormat,
) -> std::result::Result<(), String> {
    if total_pages <= 1 {
        return Ok(());
    }

    let template: String = output_path.to_string_lossy().into_owned();
    if template.contains("{p}") || template.contains("{0p}") || template.contains("{t}") {
        return Ok(());
    }

    Err(format!(
        "Multi-page {} output requires an `output` path template containing at least one of {{p}}, {{0p}}, or {{t}}. See {}.",
        format.extension(),
        format!("https://typst.app/docs/reference/{}/", format.extension())
    ))
}

#[cfg(test)]
mod tests {
    use super::render_page_template_path;
    use std::path::PathBuf;

    #[test]
    fn render_page_template_path_supports_zero_padding_and_total_pages() {
        let template: PathBuf = PathBuf::from("/tmp/out-{0p}-of-{t}.png");

        let rendered_page_3: PathBuf = render_page_template_path(&template, 3, 12);

        assert_eq!(
            rendered_page_3,
            PathBuf::from("/tmp/out-03-of-12.png"),
            "template should replace {{0p}} and {{t}}"
        );
    }
}
