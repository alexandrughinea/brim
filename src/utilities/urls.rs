use regex::Regex;

pub fn parse_file_url_list(input: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let pattern = r"[,]";
    let regex = Regex::new(pattern).unwrap();
    let segments = regex.split(input).map(|s| s.trim()).collect::<Vec<_>>();

    for segment in segments {
        if !segment.is_empty() {
            urls.push(segment.to_string());
        }
    }

    urls
}

fn is_remote_file(value: &str) -> Result<bool, String> {
    let pattern = r"^https://";
    let regex = Regex::new(pattern).unwrap();

    if regex.is_match(value) {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn is_file_path(value: &str) -> Result<bool, String> {
    let pattern = r"^(\/[\w\-]+)+\/([\w\-.]+(\.\w+)?)?$";
    let regex = Regex::new(pattern).unwrap();

    // Test if the given path matches the regex pattern
    if regex.is_match(value) {
        if let Some(captures) = regex.captures(value) {
            if let Some(extension) = captures.get(3) {
                let extension_str = extension.as_str();
                if extension_str == "" || extension_str == "." {
                    return Err("Invalid file extension".to_string());
                }
            }
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn parse_input_string(value: &str) {
    match (is_remote_file(value).is_ok(), is_file_path(value).is_ok()) {
        (true, _) => eprint!("is remote file"),
        (_, true) => eprint!("is file path"),
        _ => eprint!("don't know what this is"),
    }
}