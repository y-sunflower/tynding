use typst_pdf::{PdfStandard, PdfStandards};

pub fn parse_pdf_standards(value: &str) -> std::result::Result<PdfStandards, String> {
    let mut list: Vec<PdfStandard> = Vec::new();

    for raw in value.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let lower: String = raw.to_ascii_lowercase();
        let standard: PdfStandard = match lower.as_str() {
            "1.4" => PdfStandard::V_1_4,
            "1.5" => PdfStandard::V_1_5,
            "1.6" => PdfStandard::V_1_6,
            "1.7" => PdfStandard::V_1_7,
            "2.0" => PdfStandard::V_2_0,
            "a-1b" => PdfStandard::A_1b,
            "a-1a" => PdfStandard::A_1a,
            "a-2b" => PdfStandard::A_2b,
            "a-2u" => PdfStandard::A_2u,
            "a-2a" => PdfStandard::A_2a,
            "a-3b" => PdfStandard::A_3b,
            "a-3u" => PdfStandard::A_3u,
            "a-3a" => PdfStandard::A_3a,
            "a-4" => PdfStandard::A_4,
            "a-4f" => PdfStandard::A_4f,
            "a-4e" => PdfStandard::A_4e,
            "ua-1" => PdfStandard::Ua_1,
            "ua-2" => return Err("This Typst version does not support PDF/UA-2".to_owned()),
            _ => return Err(format!("Unsupported PDF standard: {raw}")),
        };
        list.push(standard);
    }

    PdfStandards::new(&list).map_err(|err| format!("Invalid PDF standard combination: {err}"))
}
