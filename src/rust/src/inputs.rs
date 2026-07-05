use typst::foundations::{Dict, IntoValue};

fn parse_sys_input_pair(raw: &str) -> std::result::Result<(String, String), String> {
    let (key, val) = raw
        .split_once('=')
        .ok_or_else(|| "input must be a key and a value separated by an equal sign".to_owned())?;

    let key: String = key.trim().to_owned();
    if key.is_empty() {
        return Err("the key was missing or empty".to_owned());
    }

    Ok((key, val.trim().to_owned()))
}

pub fn build_sys_inputs(raw_inputs: Option<&[String]>) -> std::result::Result<Dict, String> {
    let mut inputs: Dict = Dict::new();

    if let Some(raw_inputs) = raw_inputs {
        for raw in raw_inputs {
            let (key, value) = parse_sys_input_pair(raw)?;
            // sys inputs arrive from R as `key=value` strings. Keep values as
            // Typst strings rather than parsing arbitrary Typst expressions so
            // the R API remains predictable and side-effect free.
            inputs.insert(key.as_str().into(), value.into_value());
        }
    }

    Ok(inputs)
}
