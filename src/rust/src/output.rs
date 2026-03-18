use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputFormat {
    Pdf,
    Html,
    Png,
    Svg,
}

impl OutputFormat {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Html => "html",
            Self::Png => "png",
            Self::Svg => "svg",
        }
    }
}

fn parse_output_format(value: &str) -> std::result::Result<OutputFormat, String> {
    let normalized: String = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err("`output_format` must not be empty".to_owned());
    }

    match normalized.as_str() {
        "pdf" => Ok(OutputFormat::Pdf),
        "html" => Ok(OutputFormat::Html),
        "png" => Ok(OutputFormat::Png),
        "svg" => Ok(OutputFormat::Svg),
        _ => Err(format!(
            "Unsupported output format: {value}. Supported formats are: pdf, html, png, svg"
        )),
    }
}

pub fn infer_output_format(
    output_path: Option<&Path>,
    output_format: Option<&str>,
) -> std::result::Result<OutputFormat, String> {
    if let Some(value) = output_format {
        return parse_output_format(value);
    }

    let Some(path) = output_path else {
        return Ok(OutputFormat::Pdf);
    };

    let Some(ext) = path.extension().and_then(|ext| ext.to_str()) else {
        return Ok(OutputFormat::Pdf);
    };

    parse_output_format(ext).map_err(|_| {
        format!(
            "Could not infer output format from output path: {}. Supported formats are: pdf, html, png, svg. Pass `output_format` explicitly.",
            path.display()
        )
    })
}
