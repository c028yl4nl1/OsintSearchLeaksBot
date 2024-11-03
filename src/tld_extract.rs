use tldextract::*;
pub fn extract_tld(host: &str) -> Option<String> {
    let option = TldOption::default();
    let v = TldExtractor::new(option).extract(&host.replace(" ", ""));
    match v {
        Ok(url) => {
            let format_website = format!("{}.{}", url.domain.unwrap(), url.suffix.unwrap());
            Some(format_website)
        }

        _ => None,
    }
}

pub fn extract_tld_suffix(host: &str) -> Option<String> {
    let option = TldOption::default();
    let v = TldExtractor::new(option).extract(&host.replace(" ", ""));
    match v {
        Ok(url) => {
            let format_website = url.suffix.unwrap_or("com".to_string());
            Some(format_website)
        }

        _ => None,
    }
}
